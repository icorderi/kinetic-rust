#![macro_escape]

#[macro_export]
macro_rules! cmd( ($name:ident, $execute:ident, $usage: ident) => (
    impl ::cli::CliCommand for $name {
        fn from_argv(argv: ::std::vec::Vec<String>) -> $name {
            ::docopt::Docopt::new(::cli::CliCommand::usage(None::<$name>))
                .and_then(|d| d.argv(argv.clone().into_iter()).decode() )
                .unwrap_or_else(|e| e.exit())
        }

        #[inline]
        fn execute(&self, shell: &mut ::shell::MultiShell) -> ::kinetic::KineticResult<()> {
            $execute(self, shell)
        }

        #[inline]
        fn usage(_: Option<$name>) -> &'static str { $usage }
    }
));
