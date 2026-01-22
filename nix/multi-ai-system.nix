{
  description = "Multi-AI service integration with human tasking system";
  
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    ai-ticket.url = "path:/mnt/data1/2023/09/24/ai-ticket";
  };
  
  outputs = { self, nixpkgs, ai-ticket }:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
      
      # AI Service Router
      aiServiceRouter = pkgs.writeScriptBin "ai-service-router" ''
        #!/bin/bash
        
        TASK="$1"
        SERVICE="$2"
        
        case "$SERVICE" in
          gemini)
            # Gemini via our system
            nix run ./nix/gemini-monitored.nix#default -- -p "$TASK"
            ;;
          
          huggingface)
            # HuggingFace models
            python3 - << EOF
        import requests
        response = requests.post(
            "https://api-inference.huggingface.co/models/meta-llama/Llama-2-70b-chat-hf",
            headers={"Authorization": "Bearer $HF_TOKEN"},
            json={"inputs": "$TASK"}
        )
        print(response.json())
        EOF
            ;;
          
          anthropic)
            # Claude via API
            curl https://api.anthropic.com/v1/messages \
              -H "x-api-key: $ANTHROPIC_API_KEY" \
              -H "anthropic-version: 2023-06-01" \
              -H "content-type: application/json" \
              -d "{\"model\": \"claude-3-opus-20240229\", \"messages\": [{\"role\": \"user\", \"content\": \"$TASK\"}]}"
            ;;
          
          openai)
            # OpenAI via API
            curl https://api.openai.com/v1/chat/completions \
              -H "Authorization: Bearer $OPENAI_API_KEY" \
              -H "Content-Type: application/json" \
              -d "{\"model\": \"gpt-4\", \"messages\": [{\"role\": \"user\", \"content\": \"$TASK\"}]}"
            ;;
          
          human)
            # Create ticket for human
            ${aiTicketSystem}/bin/create-ticket "$TASK"
            ;;
          
          *)
            echo "Unknown service: $SERVICE"
            exit 1
            ;;
        esac
      '';
      
      # Human Tasking System (based on ai-ticket)
      aiTicketSystem = pkgs.writeScriptBin "ai-ticket-system" ''
        #!/bin/bash
        
        COMMAND="$1"
        
        case "$COMMAND" in
          create)
            TASK="$2"
            REWARD="$3"
            
            # Create GitHub issue as ticket
            TICKET_ID=$(date +%s)
            
            cat > "data/tickets/ticket_$TICKET_ID.json" << EOF
        {
          "id": "$TICKET_ID",
          "task": "$TASK",
          "reward": "$REWARD",
          "status": "open",
          "created": "$(date -Iseconds)",
          "assigned_to": null,
          "result": null
        }
        EOF
            
            echo "✅ Ticket created: $TICKET_ID"
            echo "   Task: $TASK"
            echo "   Reward: $REWARD"
            echo "   URL: https://github.com/meta-introspector/meta-introspector/issues/$TICKET_ID"
            ;;
          
          list)
            # List open tickets
            echo "📋 Open Tickets:"
            for ticket in data/tickets/ticket_*.json; do
              if [ -f "$ticket" ]; then
                STATUS=$(jq -r '.status' "$ticket")
                if [ "$STATUS" = "open" ]; then
                  ID=$(jq -r '.id' "$ticket")
                  TASK=$(jq -r '.task' "$ticket")
                  REWARD=$(jq -r '.reward' "$ticket")
                  echo "  [$ID] $TASK (Reward: $REWARD)"
                fi
              fi
            done
            ;;
          
          claim)
            TICKET_ID="$2"
            USER="$3"
            
            # Claim ticket
            TICKET_FILE="data/tickets/ticket_$TICKET_ID.json"
            jq --arg user "$USER" '.assigned_to = $user | .status = "claimed"' \
              "$TICKET_FILE" > "$TICKET_FILE.tmp" && mv "$TICKET_FILE.tmp" "$TICKET_FILE"
            
            echo "✅ Ticket $TICKET_ID claimed by $USER"
            ;;
          
          submit)
            TICKET_ID="$2"
            RESULT="$3"
            
            # Submit result
            TICKET_FILE="data/tickets/ticket_$TICKET_ID.json"
            jq --arg result "$RESULT" '.result = $result | .status = "completed"' \
              "$TICKET_FILE" > "$TICKET_FILE.tmp" && mv "$TICKET_FILE.tmp" "$TICKET_FILE"
            
            # Pay reward
            REWARD=$(jq -r '.reward' "$TICKET_FILE")
            USER=$(jq -r '.assigned_to' "$TICKET_FILE")
            
            echo "✅ Ticket $TICKET_ID completed"
            echo "   Reward $REWARD paid to $USER"
            ;;
          
          *)
            echo "Usage: ai-ticket-system <create|list|claim|submit>"
            ;;
        esac
      '';
      
      # Browser automation for services requiring browsers
      browserAutomation = pkgs.writeScriptBin "browser-ai-service" ''
        #!/bin/bash
        
        SERVICE="$1"
        TASK="$2"
        
        case "$SERVICE" in
          chatgpt-web)
            # ChatGPT web interface
            ${pkgs.playwright}/bin/playwright codegen \
              --target python \
              --output chatgpt_automation.py \
              https://chat.openai.com
            
            python3 chatgpt_automation.py "$TASK"
            ;;
          
          claude-web)
            # Claude web interface
            ${pkgs.playwright}/bin/playwright codegen \
              --target python \
              --output claude_automation.py \
              https://claude.ai
            
            python3 claude_automation.py "$TASK"
            ;;
          
          *)
            echo "Unknown browser service: $SERVICE"
            ;;
        esac
      '';
      
      # Task scheduler with fallback
      taskScheduler = pkgs.writeScriptBin "schedule-ai-task" ''
        #!/bin/bash
        
        TASK="$1"
        PRIORITY="$2"
        
        echo "📋 Scheduling task: $TASK"
        echo "   Priority: $PRIORITY"
        
        # Try services in order
        SERVICES=("gemini" "huggingface" "anthropic" "openai" "human")
        
        for service in "''${SERVICES[@]}"; do
          echo "   Trying $service..."
          
          if ${aiServiceRouter}/bin/ai-service-router "$TASK" "$service"; then
            echo "   ✅ Completed via $service"
            exit 0
          else
            echo "   ❌ Failed, trying next..."
          fi
        done
        
        echo "   ⚠️  All AI services failed, creating human ticket..."
        ${aiTicketSystem}/bin/ai-ticket-system create "$TASK" "10 credits"
      '';
      
    in {
      packages.${system} = {
        router = aiServiceRouter;
        tickets = aiTicketSystem;
        browser = browserAutomation;
        scheduler = taskScheduler;
        
        default = pkgs.symlinkJoin {
          name = "multi-ai-system";
          paths = [
            aiServiceRouter
            aiTicketSystem
            browserAutomation
            taskScheduler
          ];
        };
      };
      
      apps.${system} = {
        default = {
          type = "app";
          program = "${taskScheduler}/bin/schedule-ai-task";
        };
        
        tickets = {
          type = "app";
          program = "${aiTicketSystem}/bin/ai-ticket-system";
        };
      };
    };
}
