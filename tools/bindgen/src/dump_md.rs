fn main() {
    let files: Vec<String> = std::env::args().skip(1).collect();
    let index = windows_metadata::reader::Index::new(
        files
            .iter()
            .filter_map(windows_metadata::reader::File::read)
            .collect(),
    );
    for td in index.types() {
        if td.namespace() == "Microsoft.UI.Xaml"
            && (td.name() == "IDependencyObject"
                || td.name() == "IDependencyObject2"
                || td.name() == "IFrameworkElement"
                || td.name() == "IUIElement"
                || td.name() == "ICompositor"
                || td.name() == "DependencyObject")
        {
            println!(
                "{}.{} ({} methods)",
                td.namespace(),
                td.name(),
                td.methods().count()
            );
            for m in td.methods().take(20) {
                println!("   {}", m.name());
            }
        }
        if td.namespace() == "Microsoft.UI.Composition"
            && (td.name() == "ICompositor" || td.name().contains("Compositor"))
        {
            println!(
                "{}.{} ({} methods)",
                td.namespace(),
                td.name(),
                td.methods().count()
            );
        }
    }
}
