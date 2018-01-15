pub struct Linux {
  static_extension: String,
  shared_extension: String,
}

impl Linux {
  pub fn new() -> Linux {
    Linux { static_extension: Linux::static_extension(), shared_extension: Linux::shared_extension() }
  }

  pub fn name() -> String {
    String::from("linux.tup")
  }

  pub fn static_extension() -> String {
    String::from("STATIC = a")
  }

  pub fn shared_extension() -> String {
    String::from("SHARED = so")
  }
}

impl ToString for Linux {
  fn to_string(&self) -> String {
    [self.static_extension.as_str(), self.shared_extension.as_str()].join("\n")
  }
}

pub struct Darwin {
  static_extension: String,
  shared_extension: String,
}

impl Darwin {
  pub fn new() -> Darwin {
    Darwin { static_extension: Darwin::static_extension(), shared_extension: Darwin::shared_extension() }
  }

  pub fn name() -> String {
    String::from("macosx.tup")
  }

  pub fn static_extension() -> String {
    String::from("STATIC = a")
  }

  pub fn shared_extension() -> String {
    String::from("SHARED = so")
  }
}

impl ToString for Darwin {
  fn to_string(&self) -> String {
    [self.static_extension.as_str(), self.shared_extension.as_str()].join("\n")
  }
}

pub struct Windows {
  static_extension: String,
  shared_extension: String,
}

impl Windows {
  pub fn new() -> Windows {
    Windows { static_extension: Windows::static_extension(), shared_extension: Windows::shared_extension() }
  }

  pub fn name() -> String {
    String::from("win32.tup")
  }

  pub fn static_extension() -> String {
    String::from("STATIC = lib")
  }

  pub fn shared_extension() -> String {
    String::from("SHARED = dll")
  }
}

impl ToString for Windows {
  fn to_string(&self) -> String {
    let clang_comment = "# Use clang for front-end";
    let clang = "CC = clang++.exe";
    let llvm_comment = "# Use llvm-lib for static libraries";
    let archive_macro = "!archive = |> llvm-lib /MACHINE:X64 /OUT:%o %f |>";
    
    [self.static_extension.as_str(),
     self.shared_extension.as_str(),
     clang_comment,
     clang,
     llvm_comment,
     archive_macro].join("\n")
  }
}