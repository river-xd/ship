
use tokio::*;
use clap::Parser;


use crate::{
  config::ShipConfig,
  consts::paths,
  skip_handeling
};

use std::{
  path::Path,
  collections::VecDeque,
};

#[cfg(not(target_os="windows"))]
use std::os::unix::fs::MetadataExt;



#[derive(Parser,Debug)]
pub struct Clean {
  doc: bool,
  dry_run: bool,
  quite: bool
}


impl Clean {
  pub async fn clean(self)-> io::Result<()> {
    drop(ShipConfig::fetch_config().await?);// just to make sure that root of the project is the cwd.
    let mut count=0usize;
    let mut size=0u64;
    let mut queue=VecDeque::<Box<Path>>::new();

    if self.doc {
      queue.push_back(Path::new(paths::DOC_DIR).into());
    }
    queue.push_back(Path::new(paths::TARGET_DIR).into());


    while let Some(path)=queue.pop_back() {
      let mut iter=skip_handeling!(fs::read_dir(&path).await => io::ErrorKind::NotFound => continue)?;

      while let Some(entry)=iter.next_entry().await? {
        let metadata=entry.metadata().await?;
        if metadata.is_dir() {
          queue.push_back(entry.path().into_boxed_path());
          continue;
        }

        count+=1;
        size+=metadata.size();
      }
    }

    let msg=match self.dry_run {
      true=> "Summary",
      _=> {
        skip_handeling!(fs::remove_dir_all(paths::TARGET_DIR).await => io::ErrorKind::NotFound => Ok(()))?;
        "Removed"
      }
    };

    if self.quite {
      return Ok(());
    }

    match size {
      1024.. => {
        let (bytes,unit)=human_readable_bytes(size);
        color_print::cprintln!("\t<bold><g>{}</g></bold> {count} files, {bytes:.1}{unit} total",msg)
      },
      _=> color_print::cprintln!("\t<bold><g>{}</g></bold> {count} files, {size}B total",msg)
    }
    Ok(())
  }
}


fn human_readable_bytes(bytes: u64)-> (f32,&'static str) {
  pub const UNITS: [&'static str;7]=["B","KiB","MiB","GiB","TiB","PiB","EiB"];
  let bytes=bytes as f32;
  let i=((bytes.log2()/10.) as usize).min(UNITS.len()-1);

  (bytes/1024_f32.powi(i as _),UNITS[i])
}



