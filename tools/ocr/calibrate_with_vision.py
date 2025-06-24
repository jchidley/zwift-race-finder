#!/usr/bin/env python3
"""
Zwift OCR Region Calibration Tool using Vision LLMs

This tool automatically detects UI regions in Zwift screenshots using
vision-capable LLMs (Groq's Llama 3.2 Vision or local Ollama).

Usage:
    # With Groq API (recommended - needs GROQ_API_KEY env var)
    python calibrate_with_vision.py /path/to/zwift_screenshot.png

    # With local Ollama
    python calibrate_with_vision.py /path/to/zwift_screenshot.png --provider ollama

    # Output to specific file
    python calibrate_with_vision.py screenshot.png --output my_config.json
"""

import argparse
import base64
import json
import os
import sys
from datetime import datetime
from pathlib import Path
from typing import Dict, Tuple

try:
    import numpy as np
    from PIL import Image
except ImportError:
    print('Error: Required packages not installed.')
    print('Please run: pip install pillow numpy')
    sys.exit(1)

# Expected UI elements in Zwift
ZWIFT_UI_ELEMENTS = [
    'speed',  # km/h display
    'distance',  # km traveled
    'altitude',  # current altitude in meters
    'race_time',  # elapsed time MM:SS
    'power',  # watts
    'cadence',  # rpm
    'heart_rate',  # bpm
    'gradient',  # climb percentage
    'distance_to_finish',  # km remaining (race mode)
    'leaderboard',  # rider list
    'rider_pose_avatar',  # center avatar for pose detection
]


def encode_image_to_base64(image_path: str) -> str:
    """Convert image file to base64 string."""
    with open(image_path, 'rb') as image_file:
        return base64.b64encode(image_file.read()).decode('utf-8')


def get_image_dimensions(image_path: str) -> Tuple[int, int]:
    """Get image width and height."""
    with Image.open(image_path) as img:
        return img.size


def create_detection_prompt() -> str:
    """Create the prompt for UI element detection."""
    return f"""You are analyzing a Zwift cycling game screenshot to identify UI element locations.

Please locate the following UI elements and return their bounding box coordinates:
{', '.join(ZWIFT_UI_ELEMENTS)}

For each element found, provide:
- x: left position in pixels
- y: top position in pixels  
- width: width in pixels
- height: height in pixels

Important guidelines:
1. Add 5-10 pixel padding around text elements for better OCR
2. For the leaderboard, capture the full width including rider names
3. Some elements like distance_to_finish only appear in race mode
4. The gradient indicator may move position during climbs
5. Include elements even if partially visible

Return the results as a JSON object with this structure:
{{
    "regions": {{
        "speed": {{"x": 10, "y": 20, "width": 80, "height": 40}},
        "distance": {{"x": 150, "y": 20, "width": 100, "height": 40}},
        // ... other elements
    }},
    "notes": "Any relevant observations about the UI layout"
}}

Only include elements you can clearly identify. Omit any that aren't visible."""


def calibrate_with_groq(image_path: str) -> Dict:
    """Use Groq's Llama 4 Scout Vision to detect UI regions."""
    try:
        from groq import Groq
    except ImportError:
        print('Error: groq package not installed.')
        print('Please run: pip install groq')
        sys.exit(1)

    api_key = os.environ.get('GROQ_API_KEY')
    if not api_key:
        print('Error: GROQ_API_KEY environment variable not set.')
        print('Get your free API key at: https://console.groq.com')
        sys.exit(1)

    print('Encoding image...')
    image_base64 = encode_image_to_base64(image_path)

    print('Calling Groq API for UI detection...')
    client = Groq(api_key=api_key)

    try:
        response = client.chat.completions.create(
            model='meta-llama/llama-4-scout-17b-16e-instruct',
            messages=[
                {
                    'role': 'user',
                    'content': [
                        {'type': 'text', 'text': create_detection_prompt()},
                        {
                            'type': 'image_url',
                            'image_url': {'url': f'data:image/png;base64,{image_base64}'},
                        },
                    ],
                }
            ],
            response_format={'type': 'json_object'},
            temperature=0.1,
            max_tokens=2048,
        )

        result = json.loads(response.choices[0].message.content)
        print(f'Successfully detected {len(result.get("regions", {}))} UI elements')
        return result

    except Exception as e:
        print(f'Error calling Groq API: {e}')
        sys.exit(1)


def calibrate_with_huggingface(image_path: str) -> Dict:
    """Use HuggingFace's free inference API for vision models."""
    try:
        import requests
    except ImportError:
        print('Error: requests package not installed.')
        print('Please run: pip install requests')
        sys.exit(1)

    api_key = os.environ.get('HF_TOKEN')
    if not api_key:
        print('Error: HF_TOKEN environment variable not set.')
        print('Get your free API key at: https://huggingface.co/settings/tokens')
        sys.exit(1)

    print('Encoding image...')
    image_base64 = encode_image_to_base64(image_path)

    print('Calling HuggingFace API for UI detection...')
    headers = {'Authorization': f'Bearer {api_key}'}

    # Using Llava model which is good for UI detection
    API_URL = 'https://api-inference.huggingface.co/models/llava-hf/llava-1.5-7b-hf'

    try:
        response = requests.post(
            API_URL,
            headers=headers,
            json={
                'inputs': {'image': image_base64, 'text': create_detection_prompt()},
                'parameters': {'max_new_tokens': 2048, 'temperature': 0.1},
            },
            timeout=120,
        )

        if response.status_code != 200:
            raise Exception(f'API error: {response.text}')

        # Parse the response - HF returns text that we need to extract JSON from
        text_response = response.json()[0]['generated_text']
        # Extract JSON from the response
        import re

        json_match = re.search(r'\{.*\}', text_response, re.DOTALL)
        if json_match:
            result = json.loads(json_match.group())
            print(f'Successfully detected {len(result.get("regions", {}))} UI elements')
            return result
        else:
            raise Exception('No JSON found in response')

    except Exception as e:
        print(f'Error calling HuggingFace API: {e}')
        sys.exit(1)


def calibrate_with_together(image_path: str) -> Dict:
    """Use Together AI's API with Llama Vision model."""
    try:
        import requests
    except ImportError:
        print('Error: requests package not installed.')
        print('Please run: pip install requests')
        sys.exit(1)

    api_key = os.environ.get('TOGETHER_API_KEY')
    if not api_key:
        print('Error: TOGETHER_API_KEY environment variable not set.')
        print('Get your free API key at: https://api.together.xyz')
        sys.exit(1)

    print('Encoding image...')
    image_base64 = encode_image_to_base64(image_path)

    print('Calling Together AI API for UI detection...')

    try:
        response = requests.post(
            'https://api.together.xyz/v1/chat/completions',
            headers={'Authorization': f'Bearer {api_key}', 'Content-Type': 'application/json'},
            json={
                'model': 'meta-llama/Llama-3.2-11B-Vision-Instruct-Turbo',
                'messages': [
                    {
                        'role': 'user',
                        'content': [
                            {'type': 'text', 'text': create_detection_prompt()},
                            {
                                'type': 'image_url',
                                'image_url': {'url': f'data:image/jpeg;base64,{image_base64}'},
                            },
                        ],
                    }
                ],
                'response_format': {'type': 'json_object'},
                'temperature': 0.1,
                'max_tokens': 2048,
            },
            timeout=120,
        )

        if response.status_code != 200:
            raise Exception(f'API error: {response.text}')

        result = json.loads(response.json()['choices'][0]['message']['content'])
        print(f'Successfully detected {len(result.get("regions", {}))} UI elements')
        return result

    except Exception as e:
        print(f'Error calling Together AI API: {e}')
        sys.exit(1)


def calibrate_with_ollama(image_path: str) -> Dict:
    """Use local Ollama with Llama 3.2 Vision to detect UI regions."""
    try:
        import requests
    except ImportError:
        print('Error: requests package not installed.')
        print('Please run: pip install requests')
        sys.exit(1)

    # Check if Ollama is running
    try:
        response = requests.get('http://localhost:11434/api/tags')
        if response.status_code != 200:
            raise Exception('Ollama not responding')
    except Exception:
        print('Error: Ollama is not running.')
        print('Please start Ollama: ollama serve')
        sys.exit(1)

    print('Encoding image...')
    image_base64 = encode_image_to_base64(image_path)

    print('Calling local Ollama for UI detection...')
    try:
        response = requests.post(
            'http://localhost:11434/api/generate',
            json={
                'model': 'llama3.2-vision:11b',
                'prompt': create_detection_prompt(),
                'images': [image_base64],
                'format': 'json',
                'stream': False,
                'options': {'temperature': 0.1, 'num_predict': 2048},
            },
            timeout=120,  # Vision models can be slow
        )

        if response.status_code != 200:
            raise Exception(f'Ollama API error: {response.text}')

        result = json.loads(response.json()['response'])
        print(f'Successfully detected {len(result.get("regions", {}))} UI elements')
        return result

    except Exception as e:
        print(f'Error calling Ollama: {e}')
        sys.exit(1)


def validate_regions(regions: Dict, image_width: int, image_height: int) -> Dict:
    """Validate and clean up detected regions."""
    validated = {}

    for element, bounds in regions.items():
        if element not in ZWIFT_UI_ELEMENTS:
            continue

        # Ensure all required fields exist
        if not all(k in bounds for k in ['x', 'y', 'width', 'height']):
            print(f'Warning: Skipping {element} - missing coordinates')
            continue

        # Convert to integers
        try:
            x = int(bounds['x'])
            y = int(bounds['y'])
            w = int(bounds['width'])
            h = int(bounds['height'])
        except (ValueError, TypeError):
            print(f'Warning: Skipping {element} - invalid coordinates')
            continue

        # Validate bounds
        if x < 0 or y < 0 or w <= 0 or h <= 0:
            print(f'Warning: Skipping {element} - negative/zero dimensions')
            continue

        if x + w > image_width or y + h > image_height:
            print(f'Warning: Adjusting {element} - extends beyond image')
            w = min(w, image_width - x)
            h = min(h, image_height - y)

        validated[element] = {'x': x, 'y': y, 'width': w, 'height': h}

    return validated


def create_config_file(regions: Dict, image_path: str, output_path: str) -> None:
    """Create the OCR configuration file."""
    width, height = get_image_dimensions(image_path)

    # Validate regions
    validated_regions = validate_regions(regions.get('regions', {}), width, height)

    if not validated_regions:
        print('Error: No valid regions detected')
        sys.exit(1)

    # Create configuration
    config = {
        'version': '1.0.0',
        'resolution': f'{width}x{height}',
        'zwift_version': 'unknown',  # User should update this
        'created': datetime.now().isoformat(),
        'regions': validated_regions,
        'notes': regions.get('notes', ''),
        'calibration_image': Path(image_path).name,
    }

    # Write to file
    with open(output_path, 'w') as f:
        json.dump(config, f, indent=2)

    print(f'\nConfiguration saved to: {output_path}')
    print(f'Detected {len(validated_regions)} regions:')
    for element in sorted(validated_regions.keys()):
        print(f'  - {element}')

    print('\nNext steps:')
    print("1. Update 'zwift_version' in the config file")
    print(f'2. Test with: python visual_region_mapper.py {image_path} --config {output_path}')
    print('3. Fine-tune any regions that need adjustment')


def main():
    parser = argparse.ArgumentParser(
        description='Automatically detect Zwift UI regions using Vision LLMs'
    )
    parser.add_argument('image', help='Path to Zwift screenshot PNG')
    parser.add_argument(
        '--provider',
        choices=['groq', 'huggingface', 'together', 'ollama'],
        default='groq',
        help='Vision API provider (default: groq)',
    )
    parser.add_argument(
        '--output',
        default='ocr_regions_draft.json',
        help='Output configuration file (default: ocr_regions_draft.json)',
    )

    args = parser.parse_args()

    # Validate input
    if not Path(args.image).exists():
        print(f'Error: Image file not found: {args.image}')
        sys.exit(1)

    if not args.image.lower().endswith('.png'):
        print('Warning: Expected PNG file, results may vary')

    # Run calibration
    print(f'Calibrating regions from: {args.image}')
    print(f'Using provider: {args.provider}')

    if args.provider == 'groq':
        result = calibrate_with_groq(args.image)
    elif args.provider == 'huggingface':
        result = calibrate_with_huggingface(args.image)
    elif args.provider == 'together':
        result = calibrate_with_together(args.image)
    else:
        result = calibrate_with_ollama(args.image)

    # Create configuration file
    create_config_file(result, args.image, args.output)


if __name__ == '__main__':
    main()
