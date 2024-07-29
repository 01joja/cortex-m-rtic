

// All task api:s that could be generated

// task api:s (module)
pub mod foo {
    // Used by resources
    pub use super::__rtic_internal_fooLocalResources as LocalResources;
    pub use super::__rtic_internal_fooSharedResources as SharedResources;
    pub use super::__rtic_internal_foo_Context as Context;
    
    //
    pub use super::__rtic_internal_foo_spawn as spawn;
    pub use MyMono::spawn_after;
    pub use MyMono::spawn_at;
    pub use MyMono::SpawnHandle;
    pub mod MyMono {
        pub use super::super::__rtic_internal_foo_MyMono_spawn_after as spawn_after;
        pub use super::super::__rtic_internal_foo_MyMono_spawn_at as spawn_at;
        pub use super::super::__rtic_internal_foo_MyMono_SpawnHandle as SpawnHandle;
    }
}

