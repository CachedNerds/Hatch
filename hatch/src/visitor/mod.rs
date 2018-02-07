use project::{ Project, LibraryKind, ProjectKind };
use project::build::{ Target, Config };
use deps::dependency::Dependency;
use platform::arch::Arch;

pub trait ConfigVisitor {
  fn visit(&self, config: Box<&Config>);
}

struct ConfigEntryParsingVisitor {
  name: String,
  version: String,
  kind: ProjectKind,
  compiler: String,
  compiler_flags: Vec<String>,
  linker_flags: Vec<String>,
  arch: Arch,
  target: Target,
  deps: Vec<Dependency>,
}

impl ConfigEntryParsingVisitor {
  fn new() -> ConfigEntryParsingVisitor {
    ConfigEntryParsingVisitor {
      name: String::new(),
      version: String::new(),
      kind: ProjectKind::Library(LibraryKind::Static),
      compiler: String::from("g++"),
      compiler_flags: vec![String::from("-c")],
      linker_flags: vec![String::from("-v")], 
      arch: Arch::X64,
      target: Target::Debug,
      deps: Vec::new(),
    }
  }
}

impl ConfigVisitor for ConfigEntryParsingVisitor {
  fn visit(&mut self, config: Box<&Config>) {
    self.name = config.name();
    self.version = config.version();

    if let Some(k) = config.kind() {
      self.kind = k;
    }

    if let Some(c) = config.compiler() {
      self.compiler = c;
    }

    if let Some(f) = config.compiler_flags() {
      self.compiler_flags.append(f);
    }

    if let Some(l) = config.linker_flags() {
      self.linker_flags.append(l);
    }

    if let Some(a) = config.arch() {
      self.arch = a;
    }

    if let Some(t) = config.target() {
      self.target = t;
    }

    if let Some(d) = config.deps() {
      self.deps.append(d);
    }
  }
}
