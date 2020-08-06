/// ローカルのユニークなラベル名のための構造体
pub struct Label {
    label_count: u64,
    push_count: i64,
}

impl Label {
    pub fn new() -> Self {
        Label {
            label_count: 0,
            push_count: 0,
        }
    }

    pub fn get(&mut self) -> u64 {
        let a = self.label_count;
        self.label_count += 1;
        a
    }
}

impl Label {
    /// push src
    ///
    /// スタックにsrcレジスタ・src値をプッシュする。
    /// 現在のRSP位置にsrcを書き込み、RSPを8下げる。
    /// srcがレジスタの場合、srcレジスタに操作は行わない
    pub fn push<T>(&mut self, src: T)
    where
        T: std::fmt::Display,
    {
        println!("  push {}", src);
        println!(
            "# push count {} + 8 => {}",
            self.push_count,
            self.push_count + 8
        );
        self.push_count += 8;
    }

    /// pop src
    ///
    /// スタックからsrcレジスタにポップする。
    /// 現在のRSP位置の64bitをsrcに読み込み、RSPを8上げる。
    pub fn pop<T>(&mut self, src: T)
    where
        T: std::fmt::Display,
    {
        println!("  pop {}", src);
        println!(
            "# push count {} - 8 => {}",
            self.push_count,
            self.push_count - 8
        );
        self.push_count -= 8;
    }

    /// mov dst, src
    ///
    /// dstレジスタにsrcレジスタ・src値を書き込む。
    /// srcがレジスタの場合、srcレジスタに操作は行わない
    pub fn mov<T, U>(&mut self, dst: T, src: U)
    where
        T: std::fmt::Display,
        U: std::fmt::Display,
    {
        println!("  mov {}, {}", dst, src);
    }

    /// movzx dst, src
    ///
    /// dstレジスタにsrcレジスタ・src値を符号拡張せずに書き込む。
    /// srcがレジスタの場合、srcレジスタに操作は行わない
    pub fn movzx<T, U>(&mut self, dst: T, src: U)
    where
        T: std::fmt::Display,
        U: std::fmt::Display,
    {
        println!("  movzx {}, {}", dst, src);
    }

    /// add dst, src
    ///
    /// dstレジスタにdstレジスタとsrcレジスタ・src値を足した値を書き込む。
    /// srcがレジスタの場合、srcレジスタに操作は行わない
    pub fn add<T, U>(&mut self, dst: T, src: U)
    where
        T: std::fmt::Display,
        U: std::fmt::Display,
    {
        println!("  add {}, {}", dst, src);
    }

    /// sub dst, src
    ///
    /// dstレジスタにdstレジスタとsrcレジスタ・src値を引いた値を書き込む。
    /// srcがレジスタの場合、srcレジスタに操作は行わない
    pub fn sub<T, U>(&mut self, dst: T, src: U)
    where
        T: std::fmt::Display,
        U: std::fmt::Display,
    {
        println!("  sub {}, {}", dst, src);
    }

    /// imul dst, src
    ///
    /// dstレジスタにdstレジスタとsrcレジスタ・src値を符号付きで掛けた値を書き込む。
    /// srcがレジスタの場合、srcレジスタに操作は行わない
    pub fn imul<T, U>(&mut self, dst: T, src: U)
    where
        T: std::fmt::Display,
        U: std::fmt::Display,
    {
        println!("  imul {}, {}", dst, src);
    }

    /// cqo
    ///
    /// raxレジスタの値を符号付き拡張して、rdx:raxに書き込む。
    /// 割り算の前にrdxを設定する目的で使用される。
    pub fn cqo(&mut self) {
        println!("  cqo");
    }

    /// idiv src
    ///
    /// rdx:raxレジスタの値をsrcで割り、割った値をrax、あまりをrdxに書き込む。
    /// srcがレジスタの場合、srcレジスタに操作は行わない
    pub fn idiv<T>(&mut self, src: T)
    where
        T: std::fmt::Display,
    {
        println!("  idiv {}", src);
    }

    /// cmp dst, src
    ///
    /// dstとsrcの値を比較して、結果をフラグレジスタに書き込む。
    /// dst、srcがレジスタの場合、dst、srcレジスタに操作は行わない
    pub fn cmp<T, U>(&mut self, dst: T, src: U)
    where
        T: std::fmt::Display,
        U: std::fmt::Display,
    {
        println!("  cmp {}, {}", dst, src);
    }

    /// sete src
    ///
    /// フラグレジスタの値を見て、等しい場合にsrcレジスタにバイトを書き込む。
    pub fn sete<T>(&mut self, src: T)
    where
        T: std::fmt::Display,
    {
        println!("  sete {}", src);
    }

    /// sete src
    ///
    /// フラグレジスタの値を見て、等しくない場合にsrcレジスタにバイトを書き込む。
    pub fn setne<T>(&mut self, src: T)
    where
        T: std::fmt::Display,
    {
        println!("  setne {}", src);
    }

    /// sete src
    ///
    /// フラグレジスタの値を見て、小さい場合にsrcレジスタにバイトを書き込む。
    pub fn setl<T>(&mut self, src: T)
    where
        T: std::fmt::Display,
    {
        println!("  setl {}", src);
    }

    /// sete src
    ///
    /// フラグレジスタの値を見て、小さいか等しい場合にsrcレジスタにバイトを書き込む。
    pub fn setle<T>(&mut self, src: T)
    where
        T: std::fmt::Display,
    {
        println!("  setle {}", src);
    }

    /// ret
    ///
    /// 呼び出し元にリターンする。
    pub fn ret(&mut self) {
        println!("  ret");
    }

    /// jmp .Lsrc
    ///
    /// ローカルラベルにジャンプする。
    pub fn jmp<T>(&mut self, src: T)
    where
        T: std::fmt::Display,
    {
        println!("  jmp .L{}", src);
    }

    /// je .Lsrc
    ///
    /// 等しい場合、ローカルラベルにジャンプする。
    pub fn je<T>(&mut self, src: T)
    where
        T: std::fmt::Display,
    {
        println!("  je .L{}", src);
    }

    /// call src
    ///
    /// src関数を呼び出す。
    /// rspが16の倍数でない場合、16の倍数にする処理も行う。
    pub fn call<T>(&mut self, src: T)
    where
        T: std::fmt::Display,
    {
        let n = 16 - self.push_count % 16;
        if n != 16 {
            self.sub("rsp", n);
        }
        println!("  call {}", src);
        if n != 16 {
            self.add("rsp", n);
        }
    }

    /// src:
    ///
    /// 関数ラベルを設定する。
    pub fn f_label<T>(&mut self, src: T)
    where
        T: std::fmt::Display,
    {
        println!("{}:", src);
    }

    /// .Lsrc
    ///
    /// ローカルラベルを設定する。
    pub fn l_label<T>(&mut self, src: T)
    where
        T: std::fmt::Display,
    {
        println!(".L{}:", src);
    }
}
