# Feature number 000001: template
Created by|Feature Name|Creation Date|Roadmap phase|status
---|---|---|---|---|
S-spruit|mediapool|21/5/2026|Phase 0|draft
### feature description:
create a media pool where your images stay. You should be able to just select a folder, scan through the files in said folder, get your raw images you'd
like to edit, select those and send it to your filmroll.

Rust backend owns the assets and thumbnails, vue frontend displays them.

---

### Technical requirements:
- rust
- vue
```
/mediapool_engine
|_mod.rs
|_scanner.rs
|_asset.rs
```
mod.rs -> module definition, responsible for browser creation, browser will end up living in our main.rs app
scanner.rs -> actually scans the folder
asset.rs -> the asset struct and its implementation functions.
```rs
pub struct Asset {
    pub id: Uuid,
    pub path: PathBuf,
    pub thumbnail_path: Option<PathBuf>,
}
```
features of the browser include:
- [ ] Loading the images into the appstate
- [ ] displaying loaded images
- [ ] add rating
- [ ] set ready
there should be a thumbnail cashing service somewhere too.

---

### affects modules:
Which modules affect this feature should be noted here. This helps tracking down where the feature code is located later.
- rawengine

---

### implementation notes
any notes on implementions

---

### Checklist:
- [ ] All required features are present
- [ ] Features have been tested and test report is present.
- [ ] !! does not crash any modules !!

---

### future improvements
suggest future improvements here

### test report

[000001](../test_reports/000001_template.md)
