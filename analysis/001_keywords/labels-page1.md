# Term Labeling - Page 1 (Top 30 by frequency)

Auto-labeled terms from codebase analysis (sh/rs/nix/md files).

| Frequency | Term        | Character | Keywords | Emoji | Quality |
|-----------|-------------|-----------|----------|-------|---------|
| 38        | analysis    | 🔬 CORE   | analyze  | 🔬    | ✅ GOOD |
| 31        | terms       | 📝 CORE   | word     | 📝    | ✅ GOOD |
| 24        | system      | ⚙️  CORE   | infra    | ⚙️     | ✅ GOOD |
| 24        | out         | 📤 OUTPUT | result   | 📤    | ✅ GOOD |
| 22        | self        | 🔄 META   | reflect  | 🔄    | ✅ GOOD |
| 22        | packages    | 📦 BUILD  | nix      | 📦    | ✅ GOOD |
| 19        | f           | 🔤 VAR    | file     | 🔤    | ⚠️  SHORT |
| 17        | echo        | 📢 OUTPUT | print    | 📢    | ✅ GOOD |
| 16        | txt         | 📄 FILE   | text     | 📄    | ✅ GOOD |
| 12        | name        | 🏷️  META   | label    | 🏷️     | ✅ GOOD |
| 12        | markov      | 🔗 ALGO   | chain    | 🔗    | ✅ GOOD |
| 11        | i           | 🔢 VAR    | index    | 🔢    | ⚠️  SHORT |
| 11        | from        | ⬅️  SOURCE | import   | ⬅️     | ✅ GOOD |
| 11        | extract     | 🔍 ACTION | parse    | 🔍    | ✅ GOOD |
| 11        | all         | 🌐 SCOPE  | total    | 🌐    | ✅ GOOD |
| 10        | words       | 💬 DATA   | tokens   | 💬    | ✅ GOOD |
| 10        | pkgs        | 📦 BUILD  | nixpkgs  | 📦    | ✅ GOOD |
| 9         | path        | 🛤️  FILE   | location | 🛤️     | ✅ GOOD |
| 9         | grep        | 🔎 TOOL   | search   | 🔎    | ✅ GOOD |
| 9         | code        | 💻 CORE   | source   | 💻    | ✅ GOOD |
| 9         | cat         | 📖 TOOL   | read     | 📖    | ✅ GOOD |
| 8         | ngrams      | 📊 ALGO   | sequence | 📊    | ✅ GOOD |
| 8         | in          | ⬇️  SCOPE  | within   | ⬇️     | ✅ GOOD |
| 8         | for         | 🔁 LOOP   | iterate  | 🔁    | ✅ GOOD |
| 7         | with        | 🤝 CONTEXT| using    | 🤝    | ✅ GOOD |
| 7         | ngram       | 📊 ALGO   | sequence | 📊    | ✅ GOOD |
| 7         | json        | 📋 FORMAT | data     | 📋    | ✅ GOOD |
| 7         | hook        | 🪝 BUILD  | trigger  | 🪝    | ✅ GOOD |
| 7         | commit      | 💾 GIT    | save     | 💾    | ✅ GOOD |
| 6         | transitions | 🔀 ALGO   | change   | 🔀    | ✅ GOOD |

## Character Types

- 🔬 CORE - Core functionality
- 📝 DATA - Data/content
- ⚙️  INFRA - Infrastructure
- 📤 OUTPUT - Output/results
- 🔄 META - Meta/reflection
- 📦 BUILD - Build system
- 🔤 VAR - Variables
- 🏷️  LABEL - Labels/names
- 🔗 ALGO - Algorithms
- ⬅️  SOURCE - Source/input
- 🔍 ACTION - Actions/verbs
- 🌐 SCOPE - Scope/range
- 💬 CONTENT - Content/text
- 🛤️  PATH - Paths/locations
- 🔎 TOOL - Tools/utilities
- 💻 CODE - Code/source
- 📖 READ - Reading/viewing
- 📊 STATS - Statistics/analysis
- ⬇️  CONTEXT - Context/scope
- 🔁 CONTROL - Control flow
- 🤝 RELATION - Relations/connections
- 📋 FORMAT - Data formats
- 🪝 HOOK - Hooks/triggers
- 💾 VERSION - Version control
- 🔀 FLOW - Flow/transitions

## Quality Flags

- ✅ GOOD - Production quality term
- ⚠️  SHORT - Single letter variable (acceptable in loops)
- ❌ FAKE - Fake/placeholder term (ban from production)
- 🚧 TEMP - Temporary term (should be replaced)

## Next Steps

1. Continue labeling remaining terms
2. Extract ❌ FAKE terms for pre-commit hook
3. Weight terms by frequency + documentation presence
4. Generate character embeddings from labels
