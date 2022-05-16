use ::epub;
use std::env;
use ::mdbook;
use ::mdbook_epub;
use ::tempdir;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serial_test;

use epub::doc::EpubDoc;
use std::path::{Path, PathBuf};
use std::process::Command;
use tempdir::TempDir;
use std::sync::Once;
use mdbook::renderer::RenderContext;
use mdbook::{MDBook, Renderer};
use mdbook_epub::Error;

static INIT: Once = Once::new();

fn init_logging() {
    INIT.call_once(|| {
        env_logger::init();
    });
}

/// Convenience function for compiling the dummy book into an `EpubDoc`.
fn generate_epub() -> Result< (EpubDoc, PathBuf), Error> {
    let (ctx, _md, temp) = create_dummy_book().unwrap();
    debug!("temp dir = {:?}", &temp);
    mdbook_epub::generate(&ctx)?;
    let output_file = mdbook_epub::output_filename(temp.path(), &ctx.config);
    debug!("output_file = {:?}", &output_file.display());

    // let output_file_name = output_file.display().to_string();
    match EpubDoc::new(&output_file) {
        Ok(epub) => {
            let result: (EpubDoc, PathBuf) = (epub, output_file);
            return Ok( result )},
        Err(err) => {
            error!("dummy book creation error = {}", err);
            return Err(Error::EpubDocCreate(output_file.display().to_string()))?
        },
    }
}

#[test]
#[serial]
fn output_epub_exists() {
    init_logging();
    let (ctx, _md, temp) = create_dummy_book().unwrap();

    let output_file = mdbook_epub::output_filename(temp.path(), &ctx.config);

    assert!(!output_file.exists());
    mdbook_epub::generate(&ctx).unwrap();
    assert!(output_file.exists());
}

#[test]
#[serial]
fn output_epub_is_valid() {
    init_logging();
    let (ctx, _md, temp) = create_dummy_book().unwrap();
    mdbook_epub::generate(&ctx).unwrap();

    let output_file = mdbook_epub::output_filename(temp.path(), &ctx.config);

    let got = EpubDoc::new(&output_file);

    assert!(got.is_ok());

    // also try to run epubcheck, if it's available
    epub_check(&output_file).unwrap();
}

fn epub_check(path: &Path) -> Result<(), Error> {
    init_logging();
    let cmd = Command::new("epubcheck").arg(path).output();

    match cmd {
        Ok(output) => {
            if output.status.success() {
                Ok(())
            } else {
                Err(Error::EpubCheck)
            }
        }
        Err(_) => {
            // failed to launch epubcheck, it's probably not installed
            Ok(())
        }
    }
}

#[test]
#[serial]
fn look_for_chapter_1_heading() {
    init_logging();
    debug!("look_for_chapter_1_heading...");
    let mut doc = generate_epub().unwrap();
    debug!("doc current path = {:?}", doc.1);

    let path;
    if cfg!(target_os = "linux") {
        path = Path::new("OEBPS").join("chapter_1.html"); // linux
    } else {
        path = Path::new("OEBPS/chapter_1.html").to_path_buf(); // windows with 'forward slash' /
    }
    debug!("short path = {:?}", path.display().to_string());
    debug!("full path = {:?}", &doc.1);
    let file = doc.0.get_resource_str_by_path(path);
    debug!("file = {:?}", &file);
    let content = file.unwrap();
    debug!("content = {:?}", content.len());
    assert!(content.contains("<h1>Chapter 1</h1>"));
    // assert!(!content.contains("{{#rustdoc_include")); // prepare fix link error
    // assert!(content.contains("fn main() {")); // prepare fix link error
}

#[test]
#[serial]
fn rendered_document_contains_all_chapter_files_and_assets() {
    init_logging();
    debug!("rendered_document_contains_all_chapter_files_and_assets...");
    let chapters = vec!["chapter_1.html", "rust-logo.png"];
    let mut doc = generate_epub().unwrap();
    debug!("doc current path = {:?} / {:?}", doc.0.get_current_path(), doc.1);

    for chapter in chapters {
        let path;
        if cfg!(target_os = "windows") {
            path = Path::new("OEBPS/").join(chapter); // windows with 'forward slash' /
        } else {
            path = Path::new("OEBPS").join(chapter); // linux
        }
        // let path = path.display().to_string();
        debug!("path = {}", &path.display().to_string());
        let got = doc.0.get_resource_by_path(&path);
        debug!("got = {:?}", got.is_ok());
        assert!(got.is_ok(), "{}", &path.display().to_string());
    }
}

#[test]
#[serial]
fn straight_quotes_transformed_into_curly_quotes() {
    init_logging();
    debug!("straight_quotes_transformed_into_curly_quotes...");
    let mut doc = generate_epub().unwrap();
    debug!("doc current path = {:?}", doc.1);

    let path;
    if cfg!(target_os = "linux") {
        path = Path::new("OEBPS").join("chapter_1.html"); // linux
    } else {
        path = Path::new("OEBPS/chapter_1.html").to_path_buf(); // windows with 'forward slash' /
    }
    let file = doc.0.get_resource_str_by_path(path);
    let content = file.unwrap();
    debug!("content = {:?}", content);
    assert!(content.contains("<p>“One morning, when Gregor Samsa woke from troubled dreams, he found himself ‘transformed’ in his bed into a horrible vermin.”</p>"));
}


/// Use `MDBook::load()` to load the dummy book into memory, then set up the
/// `RenderContext` for use the EPUB generator.
fn create_dummy_book() -> Result<(RenderContext, MDBook, TempDir), Error> {
    let temp = TempDir::new("mdbook-epub")?;

    let dummy_book = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("dummy");
    debug!("dummy_book = {:?}", &dummy_book.display().to_string());

    let md = MDBook::load(dummy_book);

    let book = md.expect("dummy MDBook is not loaded");
    let ctx = RenderContext::new(
        book.root.clone(),
        book.book.clone(),
        book.config.clone(),
        temp.path().to_path_buf(),
    );

    Ok((ctx, book, temp))
}


#[derive(Default)]
/// A renderer to output the Markdown after the preprocessors have run. Mostly useful
/// when debugging preprocessors.
pub struct EpubRenderer;

impl EpubRenderer {
    /// Create a new `EpubRenderer` instance.
    pub fn new() -> Self {
        EpubRenderer
    }
}

impl Renderer for EpubRenderer {
    fn name(&self) -> &str {
        "epub"
    }

    fn render(&self, ctx: &RenderContext) -> Result<(), mdbook::errors::Error> {
        mdbook_epub::generate(ctx).expect("epub failed to generate");
        Ok(())
    }
}

/// Use `MDBook::load()` to load the dummy book into memory
/// Insert the renderer into the processing pipeline (as it's not "installed")
/// Call the normal mdbook build (so the default preprocessors run)
/// And load back with Epubdoc
fn generate_dummy_book() -> Result<EpubDoc, Error> {
    let temp = TempDir::new("mdbook-epub")?;

    let dummy_book = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("dummy");
    debug!("dummy_book = {:?}", &dummy_book.display().to_string());

    let md = MDBook::load(dummy_book);

    let mut book = md.expect("dummy MDBook is not loaded");
    book.with_renderer(EpubRenderer);
    book.config.build.build_dir = temp.into_path();
    book.build().expect("Build Failed");

    let outdoc = book.config.build.build_dir.join("epub").join("DummyBook.epub");

    let doc = EpubDoc::new(outdoc);
    let doc = doc.unwrap();

    Ok(doc)
}


#[test]
fn chapter_15_sample() {
    init_logging();
    debug!("chapter_15_sample...");
    let mut doc = generate_dummy_book().unwrap();

    let ch15_path = Path::new("OEBPS/ch15-01-box.html").to_path_buf(); // Manual construction as windows doesnt 'forward slash'
    let ch15_file = doc.get_resource_str_by_path(ch15_path);
    let ch15_content = ch15_file.unwrap();
    debug!("content = {:?}", ch15_content);
    assert!(ch15_content.contains("<title>Using Box&lt;T&gt; to Point to Data on the Heap</title>")); // Is the title rendering ok
    assert!(ch15_content.contains("<h2>Using <code>Box&lt;T&gt;</code> to Point to Data on the Heap</h2>")); // Is the title rendering ok
    assert!(ch15_content.contains("<code class=\"language-rust,ignore,does_not_compile\">enum List {"));  // Make sure code is rendering
    assert!(!ch15_content.contains("# fn main() {}"));  // Make sure hidden code isn't rendered
    debug!("ch15_content = {:?}", ch15_content);

    let toc_path = Path::new("OEBPS/toc.ncx").to_path_buf(); // Manual construction as windows doesnt 'forward slash'
    let toc_file = doc.get_resource_str_by_path(toc_path);
    let toc_content = toc_file.unwrap();
    assert!(toc_content.contains("<text>2. Using Box&lt;T&gt; to Point to Data on the Heap</text>"));
    debug!("toc_content = {:?}", toc_content);

    let nav_path = Path::new("OEBPS/nav.xhtml").to_path_buf(); // Manual construction as windows doesnt 'forward slash'
    let nav_file = doc.get_resource_str_by_path(nav_path);
    let nav_content = nav_file.unwrap();
    assert!(nav_content.contains("<li><a href=\"ch15-01-box.html\">2. Using Box&lt;T&gt; to Point to Data on the Heap</a></li>"));
    debug!("nav_content = {:?}", nav_content);
}