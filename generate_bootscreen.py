#!/usr/bin/env python3
"""Generate raw framebuffer data with centered cross ASCII art"""

WIDTH = 1280
HEIGHT = 720
SCALE = 4  # Each braille dot becomes 4x4 pixels

ascii_art = [
    "⠀⠀⠀⠀⢀⡠⣾⣳⡀⠀⠀⠀⠀⠀",
    "⠀⠀⡀⠀⠚⢿⣿⣿⡿⠙⠀⠀⠀⠀",
    "⠀⣘⣿⣇⡀⢘⣿⣿⠀⢀⣠⣶⡀⠀",
    "⠺⣿⣷⣝⣾⣿⣿⣿⣿⣿⣹⣷⣿⠆",
    "⠀⠘⠟⠁⠀⠀⣿⣟⠀⠀⠙⠿⠁⠀",
    "⠀⠀⠀⠀⠀⠀⣿⣿⠀⠀⠀⠀⠀⠀",
    "⠀⠀⠀⠀⠀⢠⣿⣿⠀⠀⠀⠀⠀⠀",
    "⠀⠀⠀⠀⠀⢸⣿⡿⡄⠀⠀⠀⠀⠀",
    "⠀⠀⠀⠠⣖⣿⣿⣻⡷⡄⠀⠀⠀⠀",
    "⠀⠀⠀⠀⠀⠈⢻⡟⠁⠀⠀⠀⠀⠀"
]

def braille_to_pixels(char):
    """Convert braille character to 2x4 pixel grid"""
    code = ord(char)
    if code < 0x2800 or code > 0x28FF:
        return [0] * 8
    
    offset = code - 0x2800
    pixels = []
    for i in range(8):
        pixels.append((offset >> i) & 1)
    return pixels

def main():
    # Create framebuffer (all zeros = black)
    framebuffer = bytearray(WIDTH * HEIGHT * 4)
    
    # Calculate art dimensions
    art_width = len(ascii_art[0]) * 2 * SCALE
    art_height = len(ascii_art) * 4 * SCALE
    
    # Center position
    start_x = (WIDTH - art_width) // 2
    start_y = (HEIGHT - art_height) // 2
    
    # Braille dot positions in 2x4 grid
    # Dots: 1,2,3,4,5,6,7,8 map to positions:
    dot_positions = [
        (0, 0), (0, 1), (0, 2), (1, 0), (1, 1), (1, 2), (0, 3), (1, 3)
    ]
    
    # Draw the art
    for line_idx, line in enumerate(ascii_art):
        for char_idx, char in enumerate(line):
            pixels = braille_to_pixels(char)
            
            for dot_idx, is_on in enumerate(pixels):
                if is_on:
                    dot_x, dot_y = dot_positions[dot_idx]
                    base_x = start_x + char_idx * 2 * SCALE + dot_x * SCALE
                    base_y = start_y + line_idx * 4 * SCALE + dot_y * SCALE
                    
                    # Draw scaled dot
                    for dy in range(SCALE):
                        for dx in range(SCALE):
                            x = base_x + dx
                            y = base_y + dy
                            
                            if 0 <= x < WIDTH and 0 <= y < HEIGHT:
                                offset = (y * WIDTH + x) * 4
                                framebuffer[offset + 0] = 0xFF  # Red
                                framebuffer[offset + 1] = 0xFF  # Green
                                framebuffer[offset + 2] = 0xFF  # Blue
                                framebuffer[offset + 3] = 0x00  # Padding
    
    # Write to file
    with open('cross_framebuffer.raw', 'wb') as f:
        f.write(framebuffer)
    
    print(f"Generated cross_framebuffer.raw")
    print(f"Size: {len(framebuffer)} bytes ({len(framebuffer) / 1024 / 1024:.2f} MB)")
    print(f"Format: {WIDTH}x{HEIGHT} RGBX (4 bytes per pixel)")
    print(f"White (0xFF 0xFF 0xFF 0x00) on black (0x00 0x00 0x00 0x00)")

if __name__ == '__main__':
    main()