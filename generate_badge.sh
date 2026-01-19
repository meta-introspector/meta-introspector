#!/bin/bash
# Generate your meta meme badge

echo "🎭 Meta Meme Badge Generator"
echo "============================"
echo ""

# Check if profile exists
if [ ! -f meta_meme_profile.json ]; then
    echo "❌ No profile found. Run first:"
    echo "   ./target/release/meta_meme_classifier"
    exit 1
fi

# Get top meme
TOP_MEME=$(jq -r '.profiles[0].identity' meta_meme_profile.json)
SCORE=$(jq -r '.profiles[0].score' meta_meme_profile.json)
TAGLINE=$(jq -r '.profiles[0].tagline' meta_meme_profile.json)

echo "🎯 Your Top Meme: $TOP_MEME (score: $SCORE)"
echo "💬 Tagline: \"$TAGLINE\""
echo ""

# Generate badge markdown
case "$TOP_MEME" in
    "Pythonista")
        BADGE="[![Meta Meme](https://img.shields.io/badge/meta--meme-pythonista-blue?style=for-the-badge&logo=python)](https://huggingface.co/datasets/introspector/meta-meme)"
        ;;
    "Rustacean")
        BADGE="[![Meta Meme](https://img.shields.io/badge/meta--meme-rustacean-orange?style=for-the-badge&logo=rust)](https://huggingface.co/datasets/introspector/meta-meme)"
        ;;
    "JavaScript d00d")
        BADGE="[![Meta Meme](https://img.shields.io/badge/meta--meme-js%20d00d-yellow?style=for-the-badge&logo=javascript)](https://huggingface.co/datasets/introspector/meta-meme)"
        ;;
    "Kernel Hacker")
        BADGE="[![Meta Meme](https://img.shields.io/badge/meta--meme-kernel%20hacker-black?style=for-the-badge&logo=linux)](https://huggingface.co/datasets/introspector/meta-meme)"
        ;;
    "C/C++ Wizard")
        BADGE="[![Meta Meme](https://img.shields.io/badge/meta--meme-c%2B%2B%20wizard-purple?style=for-the-badge&logo=cplusplus)](https://huggingface.co/datasets/introspector/meta-meme)"
        ;;
    "Gopher")
        BADGE="[![Meta Meme](https://img.shields.io/badge/meta--meme-gopher-cyan?style=for-the-badge&logo=go)](https://huggingface.co/datasets/introspector/meta-meme)"
        ;;
    "DevOps Ninja")
        BADGE="[![Meta Meme](https://img.shields.io/badge/meta--meme-devops%20ninja-red?style=for-the-badge&logo=docker)](https://huggingface.co/datasets/introspector/meta-meme)"
        ;;
    *)
        BADGE="[![Meta Meme](https://img.shields.io/badge/meta--meme-undefined-gray?style=for-the-badge)](https://huggingface.co/datasets/introspector/meta-meme)"
        ;;
esac

echo "📋 Your Badge (Markdown):"
echo ""
echo "$BADGE"
echo ""

# Save to file
echo "$BADGE" > my_meta_meme_badge.md
echo "✅ Saved to: my_meta_meme_badge.md"
echo ""

# Generate social media post
echo "📱 Share on Social Media:"
echo ""
echo "Twitter/X:"
echo "----------"
cat << EOF
Just discovered my meta meme! 🎭

I'm a $TOP_MEME (score: $SCORE)
"$TAGLINE"

What's yours? Find out: https://github.com/meta-introspector/meta-introspector

#MetaMeme #OpenSource #BootstrapDataset
EOF

echo ""
echo "Mastodon:"
echo "---------"
cat << EOF
🎭 Meta Meme Discovery!

My system identity: $TOP_MEME ($SCORE repos)
Tagline: "$TAGLINE"

Analyze your system: https://github.com/meta-introspector/meta-introspector

#MetaMeme #Reproducibility #FOSS
EOF

echo ""
echo "✅ Copy and paste to share your meme!"
