#!/usr/bin/env python3
import argparse
import subprocess
import sys

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--driven_audio', required=True)
    parser.add_argument('--source_image', required=True)
    parser.add_argument('--result_dir', required=True)
    args = parser.parse_args()
    
    # 调用 SadTalker
    cmd = [
        'python', 'inference.py',
        '--driven_audio', args.driven_audio,
        '--source_image', args.source_image,
        '--result_dir', args.result_dir,
        '--still',
        '--preprocess', 'full',
        '--enhancer', 'gfpgan'
    ]
    
    subprocess.run(cmd, check=True)

if __name__ == '__main__':
    main()
