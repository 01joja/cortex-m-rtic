pub mod foo {
    pub use super::__rtic_internal_fooLocalResources as LocalResources;   // Local resources
    pub use super::__rtic_internal_fooSharedResources as SharedResources; // Shared resources
    pub use super::__rtic_internal_foo_Context as Context;                // Task context
    pub use super::__rtic_internal_foo_spawn as spawn;                    // Software task spawn
    // Everything below is generated in monotonics
    pub use MyMono::spawn_after;
    pub use MyMono::spawn_at;
    pub use MyMono::SpawnHandle;
    pub mod MyMono {
        pub use super::super::__rtic_internal_foo_MyMono_spawn_after as spawn_after;
        pub use super::super::__rtic_internal_foo_MyMono_spawn_at as spawn_at;
        pub use super::super::__rtic_internal_foo_MyMono_SpawnHandle as SpawnHandle;
    }
}