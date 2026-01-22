{
  description = "Gemini with perf tracing, rate limiting, and sandbox";
  
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    gemini-telemetry.url = "path:/mnt/data1/nix/source/github/meta-introspector/streamofrandom/2025/10/08/hackathon/flakes/consolidated-impure-gemini-telemetry-modules";
  };
  
  outputs = { self, nixpkgs, gemini-telemetry }:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
      
      # Rate limit tracker
      rateLimitTracker = pkgs.writeScriptBin "gemini-rate-tracker" ''
        #!/bin/bash
        RATE_FILE="$HOME/.gemini-rate-limits.json"
        
        # Initialize if not exists
        if [ ! -f "$RATE_FILE" ]; then
          echo '{"requests": [], "daily_count": 0, "last_reset": "'$(date -Iseconds)'"}' > "$RATE_FILE"
        fi
        
        # Record request
        jq --arg time "$(date -Iseconds)" \
           '.requests += [{"timestamp": $time}] | .daily_count += 1' \
           "$RATE_FILE" > "$RATE_FILE.tmp" && mv "$RATE_FILE.tmp" "$RATE_FILE"
        
        # Check limits
        COUNT=$(jq -r '.daily_count' "$RATE_FILE")
        if [ "$COUNT" -gt 1000 ]; then
          echo "⚠️  Rate limit approaching: $COUNT/1500 requests today"
        fi
        
        echo "$COUNT"
      '';
      
      # Gemini with perf tracing
      geminiWithPerf = pkgs.writeScriptBin "gemini-traced" ''
        #!/bin/bash
        set -e
        
        TRACE_DIR="$HOME/.gemini-traces"
        mkdir -p "$TRACE_DIR"
        
        TIMESTAMP=$(date +%Y%m%d_%H%M%S)
        TRACE_FILE="$TRACE_DIR/gemini_$TIMESTAMP.perf.data"
        
        # Check rate limit
        RATE_COUNT=$(${rateLimitTracker}/bin/gemini-rate-tracker)
        echo "📊 Rate limit: $RATE_COUNT requests today"
        
        if [ "$RATE_COUNT" -gt 1500 ]; then
          echo "❌ Rate limit exceeded! Waiting..."
          exit 1
        fi
        
        # Record with perf
        echo "🔍 Recording perf trace: $TRACE_FILE"
        
        perf record -o "$TRACE_FILE" \
          -e 'syscalls:*' \
          -e 'sched:*' \
          --call-graph dwarf \
          nix run ${gemini-telemetry}#default --impure -- "$@"
        
        # Analyze trace
        echo "📈 Analyzing trace..."
        perf script -i "$TRACE_FILE" > "$TRACE_DIR/gemini_$TIMESTAMP.trace.txt"
        
        # Extract metrics
        SYSCALLS=$(perf script -i "$TRACE_FILE" | grep -c "syscalls:" || echo 0)
        DURATION=$(perf script -i "$TRACE_FILE" | tail -1 | awk '{print $4}')
        
        # Save metadata
        cat > "$TRACE_DIR/gemini_$TIMESTAMP.meta.json" << EOF
        {
          "timestamp": "$(date -Iseconds)",
          "trace_file": "$TRACE_FILE",
          "syscalls": $SYSCALLS,
          "duration": "$DURATION",
          "args": "$*",
          "rate_count": $RATE_COUNT
        }
        EOF
        
        echo "✅ Trace saved: $TRACE_FILE"
        echo "📊 Syscalls: $SYSCALLS"
      '';
      
      # Sandboxed Gemini with tools
      geminiSandbox = pkgs.writeScriptBin "gemini-sandbox" ''
        #!/bin/bash
        
        # Create sandbox
        SANDBOX_DIR=$(mktemp -d)
        trap "rm -rf $SANDBOX_DIR" EXIT
        
        # Provide tools in sandbox
        mkdir -p "$SANDBOX_DIR/bin"
        ln -s ${pkgs.jq}/bin/jq "$SANDBOX_DIR/bin/"
        ln -s ${pkgs.curl}/bin/curl "$SANDBOX_DIR/bin/"
        ln -s ${pkgs.git}/bin/git "$SANDBOX_DIR/bin/"
        
        # Sandbox permissions
        cat > "$SANDBOX_DIR/permissions.json" << EOF
        {
          "allowed_commands": ["jq", "curl", "git"],
          "allowed_paths": ["$SANDBOX_DIR"],
          "network": true,
          "max_memory": "2G",
          "max_cpu": "50%"
        }
        EOF
        
        echo "🔒 Sandbox created: $SANDBOX_DIR"
        echo "🛠️  Tools: jq, curl, git"
        
        # Run Gemini in sandbox with resource limits
        systemd-run --user --scope \
          -p MemoryMax=2G \
          -p CPUQuota=50% \
          env PATH="$SANDBOX_DIR/bin:$PATH" \
          ${geminiWithPerf}/bin/gemini-traced "$@"
      '';
      
      # Bug fix scheduler
      bugFixScheduler = pkgs.writeScriptBin "gemini-bug-scheduler" ''
        #!/bin/bash
        
        BUG_QUEUE="$HOME/.gemini-bug-queue.json"
        
        # Initialize queue
        if [ ! -f "$BUG_QUEUE" ]; then
          echo '{"bugs": []}' > "$BUG_QUEUE"
        fi
        
        # Add bug to queue
        if [ "$1" = "add" ]; then
          BUG_DESC="$2"
          jq --arg desc "$BUG_DESC" --arg time "$(date -Iseconds)" \
             '.bugs += [{"description": $desc, "added": $time, "status": "pending"}]' \
             "$BUG_QUEUE" > "$BUG_QUEUE.tmp" && mv "$BUG_QUEUE.tmp" "$BUG_QUEUE"
          echo "✅ Bug added to queue"
        fi
        
        # Process next bug
        if [ "$1" = "process" ]; then
          NEXT_BUG=$(jq -r '.bugs[] | select(.status == "pending") | .description' "$BUG_QUEUE" | head -1)
          
          if [ -z "$NEXT_BUG" ]; then
            echo "✅ No bugs in queue"
            exit 0
          fi
          
          echo "🐛 Processing bug: $NEXT_BUG"
          
          # Call Gemini to fix
          ${geminiSandbox}/bin/gemini-sandbox \
            -p "Fix this bug: $NEXT_BUG. Provide a JSON response with 'fix_type', 'description', 'commands', and 'files'." \
            --output-format json \
            --model gemini-2.5-flash
          
          # Mark as processed
          jq --arg desc "$NEXT_BUG" \
             '(.bugs[] | select(.description == $desc) | .status) = "processed"' \
             "$BUG_QUEUE" > "$BUG_QUEUE.tmp" && mv "$BUG_QUEUE.tmp" "$BUG_QUEUE"
        fi
        
        # Show queue
        if [ "$1" = "list" ]; then
          jq -r '.bugs[] | "\(.status): \(.description)"' "$BUG_QUEUE"
        fi
      '';
      
      # Trace analyzer
      traceAnalyzer = pkgs.writeScriptBin "analyze-gemini-traces" ''
        #!/bin/bash
        
        TRACE_DIR="$HOME/.gemini-traces"
        OUTPUT="$HOME/.gemini-analysis.json"
        
        echo "📊 Analyzing Gemini traces..."
        
        # Collect all traces
        TRACES=$(find "$TRACE_DIR" -name "*.meta.json" | sort)
        
        # Aggregate statistics
        jq -s '{
          total_requests: length,
          total_syscalls: map(.syscalls) | add,
          avg_syscalls: (map(.syscalls) | add / length),
          traces: .
        }' $TRACES > "$OUTPUT"
        
        echo "✅ Analysis saved: $OUTPUT"
        jq . "$OUTPUT"
      '';
      
    in {
      packages.${system} = {
        default = pkgs.symlinkJoin {
          name = "gemini-monitored";
          paths = [
            rateLimitTracker
            geminiWithPerf
            geminiSandbox
            bugFixScheduler
            traceAnalyzer
          ];
        };
        
        rate-tracker = rateLimitTracker;
        traced = geminiWithPerf;
        sandbox = geminiSandbox;
        bug-scheduler = bugFixScheduler;
        analyzer = traceAnalyzer;
      };
      
      apps.${system} = {
        default = {
          type = "app";
          program = "${geminiSandbox}/bin/gemini-sandbox";
        };
        
        schedule-bug = {
          type = "app";
          program = "${bugFixScheduler}/bin/gemini-bug-scheduler";
        };
        
        analyze = {
          type = "app";
          program = "${traceAnalyzer}/bin/analyze-gemini-traces";
        };
      };
    };
}
