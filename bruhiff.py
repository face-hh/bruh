import numpy as np
import struct
import sys
from PIL import Image

def png_to_bruh(input_path):
    img = Image.open(input_path).convert('RGB')
    width, height = img.size
    pixels = np.array(img)

    hex_body = "\n".join("".join(f"{p[0]:02x}{p[1]:02x}{p[2]:02x}" for p in row) for row in pixels)
    
    header = struct.pack('=II', width, height)
    output_path = input_path.rsplit('.', 1)[0] + '.bruh'
    
    with open(output_path, 'wb') as f:
        f.write(header)
        f.write(hex_body.encode('utf-8'))
    print(f"Created: {output_path} ({width}x{height})")

def bruh_to_png(input_path):
    with open(input_path, 'rb') as f:
        header = f.read(8)
        width, height = struct.unpack('=II', header)
        hex_data = f.read().translate(None, b'\n\r\t ')

    pixel_bytes = bytes.fromhex(hex_data.decode('utf-8'))

    pixel_array = np.frombuffer(pixel_bytes, dtype=np.uint8)
    pixel_array = pixel_array.reshape((height, width, 3))
    
    output_path = input_path.rsplit('.', 1)[0] + '.png'
    Image.fromarray(pixel_array).save(output_path)
    print(f"Created: {output_path} ({width}x{height})")

if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("USAGE:")
        print("  python bruhiff.py <file.png>   # Converts PNG to BRUH")
        print("  python bruhiff.py <file.bruh>  # Converts BRUH to PNG")
    else:
        file_path = sys.argv[1]
        if file_path.endswith('.png'):
            png_to_bruh(file_path)
        elif file_path.endswith('.bruh'):
            bruh_to_png(file_path)
        else:
            print("Error: Unsupported file extension. Use .png or .bruh")