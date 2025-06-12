#!/bin/bash
# Fix index.html after Dioxus build to include external dependencies

# Find the generated index.html
INDEX_FILE="$1/index.html"

if [ ! -f "$INDEX_FILE" ]; then
    echo "Error: index.html not found at $INDEX_FILE"
    exit 1
fi

# Create a temporary file
TEMP_FILE=$(mktemp)

# Read the original file and inject our dependencies
awk '
/<\/head>/ {
    print "    <!-- Tailwind CSS (generated) -->"
    print "    <link rel=\"stylesheet\" href=\"./assets/tailwind-generated.css\" />"
    print "    <!-- App styles -->"
    print "    <link rel=\"stylesheet\" href=\"./assets/style.css\" />"
    print "    <!-- Leaflet CSS -->"
    print "    <link rel=\"stylesheet\" href=\"https://unpkg.com/leaflet@1.9.4/dist/leaflet.css\" />"
    print "    <!-- Leaflet JS -->"
    print "    <script src=\"https://unpkg.com/leaflet@1.9.4/dist/leaflet.js\"></script>"
    print "    <!-- Pixi.js for WebGL rendering -->"
    print "    <script src=\"https://cdnjs.cloudflare.com/ajax/libs/pixi.js/7.2.4/pixi.min.js\"></script>"
}
/<title>/ {
    gsub(/dioxus \| ⛺/, "Leaflet WebGL Hybrid POC")
}
{print}
' "$INDEX_FILE" > "$TEMP_FILE"

# Replace the original file
mv "$TEMP_FILE" "$INDEX_FILE"

echo "Fixed index.html at $INDEX_FILE"