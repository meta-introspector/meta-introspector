# Term Labeling - Page 2 (Terms 31-60)

| Frequency | Term         | Character | Keywords  | Emoji | Quality |
|-----------|--------------|-----------|-----------|-------|---------|
| 6         | rs           | 🦀 LANG   | rust      | 🦀    | ✅ GOOD |
| 6         | pre          | ⏪ PREFIX | before    | ⏪    | ✅ GOOD |
| 6         | open         | 🔓 ACTION | access    | 🔓    | ✅ GOOD |
| 6         | nix          | ❄️  BUILD  | nixos     | ❄️     | ✅ GOOD |
| 6         | import       | 📥 ACTION | load      | 📥    | ✅ GOOD |
| 6         | as           | 🔄 ALIAS  | rename    | 🔄    | ✅ GOOD |
| 5         | wc           | 📏 TOOL   | count     | 📏    | ✅ GOOD |
| 5         | v            | 🔤 VAR    | value     | 🔤    | ⚠️  SHORT |
| 5         | suspicious   | 🚨 FLAG   | warning   | 🚨    | ✅ GOOD |
| 5         | stdenv       | 🏗️  BUILD  | standard  | 🏗️     | ✅ GOOD |
| 5         | r            | 🔤 VAR    | read      | 🔤    | ⚠️  SHORT |
| 5         | python3      | 🐍 LANG   | python    | 🐍    | ✅ GOOD |
| 5         | precommit    | 🪝 GIT    | hook      | 🪝    | ✅ GOOD |
| 5         | p            | 🔤 VAR    | param     | 🔤    | ⚠️  SHORT |
| 5         | mkdir        | 📁 ACTION | create    | 📁    | ✅ GOOD |
| 5         | l            | 🔤 VAR    | list      | 🔤    | ⚠️  SHORT |
| 5         | grams        | 📊 ALGO   | ngram     | 📊    | ✅ GOOD |
| 5         | generate     | ⚡ ACTION | create    | ⚡    | ✅ GOOD |
| 5         | cp           | 📋 TOOL   | copy      | 📋    | ✅ GOOD |
| 4         | trigrams     | 📊 ALGO   | 3gram     | 📊    | ✅ GOOD |
| 4         | sh           | 🐚 LANG   | shell     | 🐚    | ✅ GOOD |
| 4         | research     | 🔬 DOMAIN | study     | 🔬    | ✅ GOOD |
| 4         | o            | 🔤 VAR    | output    | 🔤    | ⚠️  SHORT |
| 4         | n            | 🔤 VAR    | number    | 🔤    | ⚠️  SHORT |
| 4         | markov_chain | 🔗 ALGO   | sequence  | 🔗    | ✅ GOOD |
| 4         | len          | 📏 FUNC   | length    | 📏    | ✅ GOOD |
| 4         | jq           | 🔧 TOOL   | json      | 🔧    | ✅ GOOD |
| 4         | fake         | ❌ FLAG   | mock      | ❌    | 🚨 FAKE |
| 4         | experimental | 🧪 DOMAIN | test      | 🧪    | ✅ GOOD |
| 4         | counts       | 🔢 DATA   | frequency | 🔢    | ✅ GOOD |

## New Character Types

- 🦀 LANG - Rust language
- ⏪ PREFIX - Prefix/before
- 🔓 OPEN - Open/access
- ❄️  NIX - Nix build system
- 📥 IMPORT - Import/load
- 🔄 ALIAS - Alias/rename
- 📏 MEASURE - Measurement/count
- 🚨 WARNING - Warning/suspicious
- 🏗️  STDENV - Standard environment
- 🐍 PYTHON - Python language
- 📁 MKDIR - Directory creation
- ⚡ GENERATE - Generation/creation
- 🐚 SHELL - Shell/bash
- 🔧 JSON - JSON tools
- 🧪 EXPERIMENT - Experimental/research
- 🔢 COUNT - Counting/frequency

## Quality Flags

- ❌ FAKE - **BANNED** from production code!
- 🚨 FAKE - First fake term detected at frequency 4

## Observations

1. **Language markers**: 🦀 rs, 🐍 python3, 🐚 sh
2. **Build system**: ❄️ nix, 🏗️ stdenv
3. **First fake term**: ❌ fake (frequency 4) - **BAN THIS**
4. **Short vars**: Still acceptable in context (v, r, p, l, o, n)
5. **Domain markers**: 🔬 research, 🧪 experimental
