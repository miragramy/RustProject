use std::{path::Path, fs::{File, self}};

use clap::Parser;

use crate::effects;


// Struct to hold command line arguments. Derives Parser from clap.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
   // Source absolute or relative path to the image
   #[arg(short = 's', long = "src")]
   pub src_path: String,

   #[arg(short = 'd', long = "dst")]
   // Destination absolute or relative path to save the new image
   pub dst_path: String,

   #[arg(short = 'e', long = "effect")]
   // Effect to apply
   pub effect: String,
}

impl Args {
   pub fn validate(&self) {
      let src_p = Path::new(self.src_path.as_str());

      if !src_p.exists() {
         panic!("Supplied source path is wrong or doesn't exist: {}", self.src_path);
      }

      let dst_p = Path::new(self.dst_path.as_str());

      let tmp_file = File::create(dst_p);

      match tmp_file {
         Ok(_) => { fs::remove_file(dst_p).unwrap(); },
         Err(err) => { panic!("Invalid destination path: {}, error: {}", self.dst_path, err.to_string()); }
      }

      if !effects::is_supported_effect(self.effect.as_str()) {
         panic!("Unsupported effect {}", self.effect);
      }
   }
}