pub fn initialize_puffin() {
    #[cfg(feature = "puffin")]
    puffin_profiler::profile_function!();
    puffin_profiler::set_scopes_on(true);
}
