#!/usr/bin/env python3
"""Create a simple icon for Strava API app"""

from PIL import Image, ImageDraw, ImageFont
import os

width, height = 512, 512
img = Image.new("RGB", (width, height), color="#FF6B00")

draw = ImageDraw.Draw(img)
for i in range(height):
    color_value = int(255 - (i / height) * 50)
    orange = (255, int(107 - (i / height) * 20), 0)
    draw.line([(0, i), (width, i)], fill=orange)

margin = 50
draw.ellipse(
    [margin, margin, width - margin, height - margin],
    fill="#FFFFFF",
    outline="#FF6B00",
    width=10,
)

try:
    font = ImageFont.truetype(
        "/usr/share/fonts/truetype/dejavu/DejaVuSans-Bold.ttf", 120
    )
except:
    font = ImageFont.load_default()

text = "ZRF"
bbox = draw.textbbox((0, 0), text, font=font)
text_width = bbox[2] - bbox[0]
text_height = bbox[3] - bbox[1]

x = (width - text_width) // 2
y = (height - text_height) // 2 - 20

draw.text((x + 5, y + 5), text, fill="#CCCCCC", font=font)
draw.text((x, y), text, fill="#FF6B00", font=font)

try:
    small_font = ImageFont.truetype(
        "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf", 40
    )
except:
    small_font = ImageFont.load_default()

subtitle = "Race Finder"
bbox = draw.textbbox((0, 0), subtitle, font=small_font)
subtitle_width = bbox[2] - bbox[0]
x_subtitle = (width - subtitle_width) // 2
y_subtitle = y + text_height + 20

draw.text((x_subtitle, y_subtitle), subtitle, fill="#666666", font=small_font)

output_path = os.path.join(os.path.dirname(__file__), "zwift_race_finder_icon.png")
img.save(output_path, "PNG")
print(f"✅ Icon created: {output_path}")
print("   Size: 512x512 pixels")
print("   Ready to upload to Strava!")
