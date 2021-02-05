use std::{
  fs::File,
  env,
};

//
// Need to construct a mask and translation table.
// Match from "heaviest" to "lightest".
// 
// [1,1,1]
// [1,1,1] → 🮋
// [1,1,1]
// 
// [1,1,1]
// [0,0,0] → 🬂
// [0,0,0]
// 
// etc...
// 
// This implementation may not use 3x3 mask or those exact characters, but
// something more suitable.
// 

const MATRIX_WIDTH_HEIGHT: usize = 3;
const NO_MASKS: usize = 58;

const LINETEXT1: [char; NO_MASKS] = [
  ' ',

  '🬡',
  '▗',
  '▗',
  '🬲',
  '🬹',
  '🬦',
  '🬊',
  '🬦',
  '🬚',
  '🬎',
  '🬵',
  
  '▉',
  '▉',
  '▉',
  '🬸',
  '🬸',
  '🭖',
  '▉',
  '▉',
  '🭅',
  '🭔',
  '▉',
  '▉',
  '🭃',
  
  '🮅',
  '▆',
  '█',
  ' ',

  '🮗',
  
  ' ',
  '🮂',
  '▋',
  '▂',

  '━',
  '▕',
  '┫',

  '🭊',
  '🭥',
  '🭊',
  '🭥',
  
  '🬏',
  '🬓',
  '🬄',
  '🬀',
  ' ',
  ' ',
  ' ',
  ' ',
  
  ' ',
  ' ',
  '🭢',
  '━',
  '🭇',
  
  ' ',
  '🭗',
  '🬼',
  ' ',
  
];

const LINETEXT2: [char; NO_MASKS] = [
  ' ',

  '🬃',
  '🬃',
  '🬃',
  '🬭',
  '🬉',
  '🬉',
  '🬩',
  '▉',
  '🬋',
  '🬌',
  '🬚',
  
  '▉',
  '🬴',
  '🬴',
  '▉',
  '▉',
  '▉',
  '🭡',
  '🭐',
  '▉',
  '▉',
  '🭟',
  '🭎',
  '▉',
  
  '🮅',
  '▆',
  ' ',
  '▋',

  '🮗',
  
  '🮉',
  '🮂',
  ' ',
  '▂',

  '━',
  '▏',
  '┣',

  '🭂',
  '🭓',
  '🭂',
  '🭓',
  
  ' ',
  ' ',
  ' ',
  ' ',
  '🬁',
  '🬉',
  '🬦',
  '🬏',
  
  '━',
  '━',
  '🭗',
  ' ',
  '🬼',
  
  '🭢',
  ' ',
  ' ',
  '🭇',
  
];

const MASKS: [[[u8; MATRIX_WIDTH_HEIGHT]; MATRIX_WIDTH_HEIGHT]; NO_MASKS] = [
  [
    [1,1,1],
    [1,1,1],
    [1,1,1],
  ],
  
  [
    [0,0,1],
    [1,1,0],
    [1,0,1],
  ],
  [
    [1,1,1],
    [1,1,0],
    [1,0,1],
  ],
  [
    [1,1,1],
    [1,1,0],
    [1,0,0],
  ],
  [
    [0,1,1],
    [0,1,1],
    [0,0,0],
  ],
  [
    [1,1,0],
    [0,0,0],
    [0,0,1],
  ],
  [
    [1,1,0],
    [1,0,0],
    [1,0,1],
  ],
  [
    [0,0,1],
    [1,0,0],
    [1,1,0],
  ],
  [
    [1,1,0],
    [1,0,0],
    [1,0,0],
  ],
  [
    [1,1,1],
    [0,0,0],
    [0,1,1],
  ],
  [
    [0,0,1],
    [0,0,0],
    [1,1,1],
  ],

  [
    [1,1,1],
    [1,0,0],
    [0,0,1],
  ],
  
  [
    [0,0,0],
    [0,0,0],
    [0,0,0],
  ],
  [
    [0,0,0],
    [0,0,1],
    [0,0,0],
  ],
  [
    [0,0,0],
    [0,1,1],
    [0,0,0],
  ],
  [
    [0,0,0],
    [1,0,0],
    [0,0,0],
  ],
  [
    [0,0,0],
    [1,1,0],
    [0,0,0],
  ],
  [
    [0,0,0],
    [1,0,0],
    [1,0,0],
  ],
  [
    [0,0,0],
    [0,0,1],
    [0,0,1],
  ],
  [
    [0,0,1],
    [0,0,1],
    [0,0,0],
  ],
  [
    [1,0,0],
    [1,0,0],
    [0,0,0],
  ],
  [
    [0,0,0],
    [0,0,0],
    [1,0,0],
  ],
  [
    [0,0,0],
    [0,0,0],
    [0,0,1],
  ],
  [
    [0,0,1],
    [0,0,0],
    [0,0,0],
  ],
  [
    [1,0,0],
    [0,0,0],
    [0,0,0],
  ],

  [
    [0,0,0],
    [0,0,0],
    [1,1,1],
  ],
  [
    [1,1,1],
    [0,0,0],
    [0,0,0],
  ],
  [
    [0,0,1],
    [0,0,1],
    [0,0,1],
  ],
  [
    [1,0,0],
    [1,0,0],
    [1,0,0],
  ],

  [
    [0,0,0],
    [1,1,1],
    [0,0,0],
  ],

  [
    [1,1,0],
    [1,1,0],
    [1,1,0],
  ],
  [
    [0,0,0],
    [1,1,1],
    [1,1,1],
  ],
  [
    [0,1,1],
    [0,1,1],
    [0,1,1],
  ],
  [
    [1,1,1],
    [1,1,1],
    [0,0,0],
  ],
  
  [
    [1,1,1],
    [0,0,0],
    [1,1,1],
  ],
  [
    [1,0,1],
    [1,0,1],
    [1,0,1],
  ],
  [
    [1,0,1],
    [0,0,0],
    [1,0,1],
  ],

  
  [
    [1,0,0],
    [0,0,0],
    [0,0,1],
  ],
  [
    [0,0,1],
    [0,0,0],
    [1,0,0],
  ],
  [
    [1,1,0],
    [1,0,0],
    [0,0,0],
  ],
  [
    [0,0,0],
    [1,0,0],
    [1,1,0],
  ],
  
  [
    [1,1,1],
    [1,1,1],
    [0,0,1],
  ],
  [
    [1,1,1],
    [0,1,1],
    [0,1,1],
  ],
  [
    [0,1,1],
    [0,1,1],
    [1,1,1],
  ],
  [
    [0,0,1],
    [1,1,1],
    [1,1,1],
  ],
  [
    [1,0,0],
    [1,1,1],
    [1,1,1],
  ],
  [
    [1,1,0],
    [1,1,0],
    [1,1,1],
  ],
  [
    [1,1,1],
    [1,1,0],
    [1,1,0],
  ],
  [
    [1,1,1],
    [1,1,1],
    [1,0,0],
  ],

  
  [
    [1,1,1],
    [1,0,0],
    [1,1,1],
  ],
  [
    [1,1,1],
    [1,1,0],
    [1,1,1],
  ],
  [
    [1,0,1],
    [1,0,1],
    [1,1,1],
  ],
  [
    [1,1,1],
    [0,0,1],
    [1,1,1],
  ],
  [
    [1,1,1],
    [1,0,1],
    [1,0,1],
  ],
  
  [
    [1,1,0],
    [1,1,1],
    [1,1,1],
  ],
  [
    [0,1,1],
    [1,1,1],
    [1,1,1],
  ],
  [
    [1,1,1],
    [1,1,1],
    [0,1,1],
  ],
  [
    [1,1,1],
    [1,1,1],
    [1,1,0],
  ],
];

//
//  Turns an array of RGBA (8 bits per channel) bytes into an array of 0s and 1s.
//
fn pixels_to_bitplane(buf: &[u8], width: u32, height: u32, bpp: usize) -> Vec<Vec<u8>> {
  let mut nbuf: Vec<Vec<u8>> = vec![];
  
  for y in 0..height {
    let mut line = vec![];
    for x in 0..width {
      let idx = (((y * width) + x) * (bpp as u32)) as usize;
      if idx >= buf.len() { break; }
      
      line.push(buf[idx] / 255);
    }
    nbuf.push(line);
  }

  nbuf
}

//
// Print the bitplane to verify it's actually converting correctly.
// For debugging purposes.
// 
fn dot_matrix_print(buf: Vec<Vec<u8>>) {
  for line in buf {
    for dot in line {
      print!("{}", if dot == 0 {
        " "
      } else {
        "0"
      });
    }
    println!("");
  }
}

//
// Convert the bitplane to unicode text!
// 
fn bitplane_to_linetext(buf: Vec<Vec<u8>>, width: u32, height: u32) {
  let mut shifted = 0;
  let mut x = 0;
  let mut y = 0;
  let mut printed_match = false;

  loop {
    for (idx, matrix) in MASKS.iter().enumerate() {
      //
      // If you change the matrix size, you'll need to adjust this (if it wasn't
      // obvious already).
      //
      if buf[y + 0][x] == matrix[0][0] && buf[y + 0][x + 1] == matrix[0][1] && buf[y + 0][x + 2] == matrix[0][2]
      && buf[y + 1][x] == matrix[1][0] && buf[y + 1][x + 1] == matrix[1][1] && buf[y + 1][x + 2] == matrix[1][2]
      && buf[y + 2][x] == matrix[2][0] && buf[y + 2][x + 1] == matrix[2][1] && buf[y + 2][x + 2] == matrix[2][2] {

        //
        // Text is taller than it is wide (about x2), so print two characters.
        //
        print!("{}", LINETEXT1[idx]);
        print!("{}", LINETEXT2[idx]);
        printed_match = true;
        break;
      }
    }

    //
    // Track how many pixels we have moved to the right
    //
    if !printed_match {
      x = x + 1;
      shifted = shifted + 1;
    } else {
      printed_match = false;
      x = x + (MATRIX_WIDTH_HEIGHT - shifted);
      shifted = 0;
    }

    //
    // We have moved enough to cover a whole mask space
    // so print a missing pattern character.
    // Basically for debugging. Change the " " to be a "?" or whatever you want.
    // Set to " " as the default for missing patterns.
    // 
    if shifted >= MATRIX_WIDTH_HEIGHT {
      print!("🮕");
      print!("🮕");
      //eprintln!("{}:{}", x - 3, y);
      shifted = 0;
    }

    // Know when to print a newline
    if x >= (width as usize) - MATRIX_WIDTH_HEIGHT {
      x = 0;
      y += MATRIX_WIDTH_HEIGHT;
      println!("");
    }
    if y >= (height as usize) - MATRIX_WIDTH_HEIGHT {
      break;
    }
  }
}

fn main() {
  let args: Vec<String> = env::args().collect();
  let decoder = png::Decoder::new(File::open(&args[1]).unwrap());
  let (info, mut reader) = decoder.read_info().unwrap();
  let mut buf = vec![0; info.buffer_size()];
  reader.next_frame(&mut buf).unwrap();
  let bitplane = pixels_to_bitplane(&buf, info.width, info.height, info.color_type.samples());
  bitplane_to_linetext(bitplane, info.width, info.height);
}
