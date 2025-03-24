use crate::{
    core::{core::*, types::CommandRegistry, witchrc::create_rc_default},
    modules::binds::sysinfo::maid_info,
};

/// Compress and Decompress files
pub fn file_compact(argsv: &[String]) -> i32 {
    let extensions = vec![
        ("7z", "7z x @@file", "7z a @@folder"),
        ("arj", "arj x @@file", "arj a @@folder"),
        ("bz2", "bzip2 -d @@file", "bzip2 @@folder"),
        ("cab", "cabextract @@file", "cabarc n @@folder"),
        ("deb", "dpkg -x @@file .", "dpkg -b @@folder"),
        (
            "dmg",
            "hdiutil attach @@file -mountpoint /tmp/mount",
            "hdiutil create -srcfolder @@folder",
        ),
        ("gz", "gunzip @@file", "gzip @@folder"),
        ("iso", "isoinfo -i @@file -x /", "genisoimage -o @@folder"),
        ("lz", "lzip -d @@file", "lzip @@folder"),
        ("lzma", "lzma -d @@file", "lzma @@folder"),
        ("lzo", "lzop -d @@file", "lzop @@folder"),
        ("lzop", "lzop -d @@file", "lzop @@folder"),
        ("rar", "unrar x @@file", "rar a @@folder"),
        (
            "rpm",
            "rpm2cpio @@file | cpio -idmv",
            "rpmbuild -ba @@folder",
        ),
        ("tar", "tar -xvf @@file", "tar -cvf @@folder"),
        ("tar.bz2", "tar -xvjf @@file", "tar -cvjf @@folder"),
        ("tar.gz", "tar -xvzf @@file", "tar -cvzf @@folder"),
        ("tar.xz", "tar -xvJf @@file", "tar -cvJf @@folder"),
        ("xz", "unxz @@file", "xz @@folder"),
        ("Z", "uncompress @@file", "compress @@folder"),
        ("zip", "unzip @@file", "zip -r @@folder"),
    ];

    let option = search_value("type", &argsv);
    let mut command = String::new();

    if option == "decompress" {
        let file = search_value("file", &argsv);
        for (ext, decomp, _comp) in extensions {
            if file.ends_with(ext) {
                command = lazy_parser(decomp, &argsv);
            }
        }
    } else {
        let format = search_value("ext", &argsv);
        for (ext, _decomp, comp) in extensions {
            if ext == format {
                command = lazy_parser(comp, &argsv);
                println!("{}", command);
            }
        }
    }
    lazy_exec(command)
}



pub fn api() -> CommandRegistry {
    vec![
        ("file.zz", file_compact),
        ("maidz", maid_info),
        ("witchrc", create_rc_default),
    ]
}
