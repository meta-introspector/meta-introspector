#!/bin/bash
# Generate FOAF (Friend of a Friend) identity document

echo "🔗 FOAF Identity Generator"
echo "=========================="
echo ""

# Get meta meme profile
if [ ! -f meta_meme_profile.json ]; then
    echo "❌ Run ./target/release/meta_meme_classifier first"
    exit 1
fi

IDENTITY=$(jq -r '.profiles[0].identity' meta_meme_profile.json)
SCORE=$(jq -r '.profiles[0].score' meta_meme_profile.json)
TAGLINE=$(jq -r '.profiles[0].tagline' meta_meme_profile.json)
EVIDENCE_HASH=$(jq -r '.profiles[0].evidence_hash // "unknown"' meta_meme_profile.json 2>/dev/null || echo "unknown")

echo "🎭 Identity: $IDENTITY"
echo "📊 Score: $SCORE"
echo ""

# Get GPG key info
GPG_KEY=$(gpg --list-secret-keys --keyid-format LONG 2>/dev/null | grep sec | head -1 | awk '{print $2}' | cut -d'/' -f2)
GPG_FINGERPRINT=$(gpg --fingerprint "$GPG_KEY" 2>/dev/null | grep -A1 "Key fingerprint" | tail -1 | tr -d ' ')

# Get SSH keys
SSH_KEY_GITHUB=$(cat ~/.ssh/id_ed25519.pub 2>/dev/null || echo "NOT_FOUND")
SSH_KEY_GITLAB=$(cat ~/.ssh/id_rsa.pub 2>/dev/null || echo "NOT_FOUND")

# Get email hash
EMAIL_HASH=$(echo -n "${EMAIL:-user@example.com}" | sha1sum | awk '{print $1}')

# Prompt for Solana wallet
read -p "Enter your Solana wallet address (or press Enter to skip): " SOLANA_WALLET
SOLANA_WALLET=${SOLANA_WALLET:-"YOUR_SOLANA_ADDRESS"}

# Generate FOAF document
cat > identity.ttl << EOF
@prefix foaf: <http://xmlns.com/foaf/0.1/> .
@prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix meta: <https://huggingface.co/datasets/introspector/meta-meme#> .
@prefix sol: <https://solana.com/> .
@prefix dc: <http://purl.org/dc/elements/1.1/> .

<#me> a foaf:Person ;
    foaf:name "Meta-Introspector" ;
    foaf:mbox_sha1sum "$EMAIL_HASH" ;
    dc:created "$(date -Iseconds)" ;
    
    # Meta Meme Identity
    meta:identity "$IDENTITY" ;
    meta:score "$SCORE" ;
    meta:tagline "$TAGLINE" ;
    meta:evidenceHash "$EVIDENCE_HASH" ;
    meta:badge <https://huggingface.co/datasets/introspector/meta-meme> ;
    
    # Solana Identity
    sol:wallet "$SOLANA_WALLET" ;
    sol:nft <https://pump.fun/SOLFUNMEME> ;
    sol:creationStory <file:///mnt/data1/nix/time/2025/01/18/SOLFUNMEME/creation.md> ;
    
    # GPG Identity
    foaf:key [
        a foaf:PGPKey ;
        foaf:keyID "$GPG_KEY" ;
        foaf:fingerprint "$GPG_FINGERPRINT" ;
        foaf:keyLocation <https://keys.openpgp.org/vks/v1/by-fingerprint/$GPG_FINGERPRINT>
    ] ;
    
    # SSH Keys
    foaf:sshPublicKey [
        rdfs:label "GitHub SSH Key" ;
        rdf:value "$SSH_KEY_GITHUB"
    ] ;
    
    foaf:sshPublicKey [
        rdfs:label "GitLab SSH Key" ;
        rdf:value "$SSH_KEY_GITLAB"
    ] ;
    
    # Git Repositories
    foaf:account [
        a foaf:OnlineAccount ;
        foaf:accountServiceHomepage <https://github.com> ;
        foaf:accountName "meta-introspector" ;
        foaf:homepage <https://github.com/meta-introspector/meta-introspector>
    ] ;
    
    # HuggingFace
    foaf:account [
        a foaf:OnlineAccount ;
        foaf:accountServiceHomepage <https://huggingface.co> ;
        foaf:accountName "introspector" ;
        foaf:homepage <https://huggingface.co/introspector>
    ] ;
    
    # Social Media Accounts
    foaf:account [
        a foaf:OnlineAccount ;
        foaf:accountServiceHomepage <https://twitter.com> ;
        foaf:accountName "introsp3ctor" ;
        foaf:homepage <https://twitter.com/introsp3ctor/>
    ] ;
    
    foaf:account [
        a foaf:OnlineAccount ;
        foaf:accountServiceHomepage <https://t.me> ;
        foaf:accountName "introsp3ctor" ;
        foaf:homepage <https://t.me/introsp3ctor>
    ] ;
    
    foaf:account [
        a foaf:OnlineAccount ;
        foaf:accountServiceHomepage <https://discord.gg> ;
        foaf:accountName "WASKdrBBzu" ;
        foaf:homepage <https://discord.gg/WASKdrBBzu>
    ] ;
    
    foaf:account [
        a foaf:OnlineAccount ;
        foaf:accountServiceHomepage <https://linkedin.com> ;
        foaf:accountName "jamesmikedupont" ;
        foaf:homepage <https://www.linkedin.com/in/jamesmikedupont>
    ] ;
    
    foaf:account [
        a foaf:OnlineAccount ;
        foaf:accountServiceHomepage <https://linktr.ee> ;
        foaf:accountName "h4km" ;
        foaf:homepage <https://linktr.ee/h4km>
    ] ;
    
    foaf:account [
        a foaf:OnlineAccount ;
        foaf:accountServiceHomepage <https://codeberg.org> ;
        foaf:accountName "introspector" ;
        foaf:homepage <https://codeberg.org/introspector/SOLFUNMEME>
    ] ;
    
    # Projects
    foaf:currentProject <https://github.com/meta-introspector/meta-introspector> ;
    foaf:currentProject <https://huggingface.co/datasets/introspector/meta-meme> ;
    foaf:currentProject <https://pump.fun/SOLFUNMEME> ;
    foaf:currentProject <https://codeberg.org/introspector/SOLFUNMEME> ;
    foaf:currentProject <https://github.com/meta-introspector/introspector-llc> ;
    
    # NFTs and Tokens
    sol:token "TSLvdd1pWpHVjahSpsvCXUbgwsL3JAcvokwaKt1eokM" ;
    sol:creator "HMEKzpgzJEfyYyqoob5uGHR9P3LF6248zbm8tWgaApim" ;
    sol:associatedToken "Ek9wkpYvXjuJDeL3CCrTZLaLE37avKgxnvYk9cSUNFK9" ;
    sol:streamflowLock <https://app.streamflow.finance/contract/solana/mainnet/7Hny19uRWs6FhWFXrasUbqkE4rc8ciTdfQ2iyr2PVeva> ;
    
    # OpenSea NFTs
    foaf:made <https://opensea.io/assets/base/0x0f4a030f9286ad80e36cd77636df8c7940c9c1b7/1> ;
    foaf:made <https://coinmarketcap.com/nft/collections/base/0x81635719fafbde22f56e0ae57511fde658a4fad9/Meta-Introspector:%20The%20Mycorrhizal%20Arboreal%20Meme%20of%20Zero%20Ontology%20System/activity/> ;
    
    # Interests
    foaf:interest <http://dbpedia.org/resource/Reproducible_builds> ;
    foaf:interest <http://dbpedia.org/resource/Zero-knowledge_proof> ;
    foaf:interest <http://dbpedia.org/resource/Blockchain> ;
    foaf:interest <http://dbpedia.org/resource/Meme> ;
    foaf:interest <http://dbpedia.org/resource/Semantic_Web> .
EOF

echo "✅ Generated: identity.ttl"
echo ""

# Validate if rapper is available
if command -v rapper &> /dev/null; then
    echo "🔍 Validating RDF..."
    if rapper -i turtle -o ntriples identity.ttl > /dev/null 2>&1; then
        echo "✅ FOAF document is valid!"
    else
        echo "⚠️  FOAF document has warnings"
    fi
else
    echo "💡 Install rapper to validate: apt install raptor2-utils"
fi

echo ""
echo "🔗 Next steps:"
echo "   1. Sign with Solana: echo 'meta-meme:$IDENTITY:$SCORE:$EVIDENCE_HASH' | solana-keygen sign"
echo "   2. Sign with GPG: gpg --clearsign identity.ttl"
echo "   3. Publish: Upload identity.ttl.asc to your website/IPFS"
echo "   4. Verify: ./verify_identity.sh"
