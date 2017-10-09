pub use self::library::{ LibraryKind };
pub use self::project::{ Project, ProjectKind };
pub use self::manifest::{ Manifest, ProjectManifest };
pub use self::platform::{ PlatformKind };
//pub use version::{ Version };

pub mod library;
pub mod project;
pub mod manifest;
pub mod platform;

//pub mod version;
