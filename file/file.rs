use std::{fs::read_dir, io::Result};


fn main() -> Result<()> {

    // SIMPLE VERSION
    // 
    // let items = std::fs::read_dir(".")?;
    //
    // for item in items {
    //  println!("{}", item?.path().display());
    // }
    //
    // Ok(())
    
    // VERBOSE VERSION 
    //
    // match read_dir(".") {
    //    Err(e) => return Err(e),
    //    Ok(itens) => {
    //        for item in itens {
    //            println!("{}", item.unwrap().path().display());
    //        }
    //    }
    // }
    //
    //Ok(())
    
    
    // MATCH ITEM AND RETURN
    for item in read_dir(".")? {
        if let Ok(i) = item {
            println!("{}", i.path().display());
        }
    }
    Ok(())
}
