pub mod linux {
    pub fn name() -> String {
        String::from("linux.tup")
    }

    pub fn to_string() -> String {
        let static_extension = "STATIC = a";
        let shared_extension = "SHARED = so";
        [
            static_extension,
            shared_extension,
        ].join("\n")
    }
}

pub mod mac_os {

    pub fn name() -> String {
        String::from("macosx.tup")
    }

    pub fn to_string() -> String {
        let static_extension ="STATIC = a";
        let shared_extension = "SHARED = so";
        [
            static_extension,
            shared_extension,
        ].join("\n")
    }
}

pub mod windows {
    pub fn name() -> String {
        String::from("win32.tup")
    }

    pub fn to_string() -> String {
        let static_extension = "STATIC = lib";
        let shared_extension = "SHARED = dll";
        let clang_comment = "# Use clang for front-end";
        let clang = "CC = clang++.exe";
        let llvm_comment = "# Use llvm-lib for static libraries";
        let archive_macro = "!archive = |> llvm-lib /MACHINE:X64 /OUT:%o %f |>";

        [
            static_extension,
            shared_extension,
            clang_comment,
            clang,
            llvm_comment,
            archive_macro,
        ].join("\n")
    }
}
