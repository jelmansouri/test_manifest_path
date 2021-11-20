pub fn manifest_dir() -> &'static str {
    #[cfg(feature = "gen_files")]
    return "gen_files";

    #[cfg(not(feature = "gen_files"))]
    return env!("CARGO_MANIFEST_DIR");
}
