# simple_config
## 使い方
* config.scfg を用意する

```
# キー: 値で並べていく、コメントは # から始める,
# 最後の行以外は文末に ,(カンマ) が必要,
input_file: hello.png,
output_file: hello.rs,
# 空行があっても大丈夫 (カンマ不要),

lang: my_lang,

# 最後の行の文末に , があっても大丈夫,
```

* Cargo.toml にsimple_configを追加する

``` toml
[dependencies]
simple_config = { git = "https://github.com/Guru521/simple_config" }
```

* ファイルの内容を <code>simple_config::scfg::parser::parse()</code> に渡して、その後キーから値を取得する
``` rust
use simple_config::scfg::parser;

fn main() {
    let configs = parser::parse(
        &fs::read_to_string("./config.scfg")
            .unwrap_or_else(|e| panic!("config file not found.(config.scfg)\n{}", e)),
    );
    
    let input_file = configs.get("input_file").unwrap();
    let output_file = configs.get("output_file").unwrap();
    let language = configs.get("lang").unwrap();
}
```

# 注意点
```
# キー = 値 は使えません,
# =? ,
key = value,

# 最後の行以外で文末のカンマを忘れるとダメ,
# ,? ,
key: value

# 数字は文字列扱いです,
# num is "100",
num: 100,
```
