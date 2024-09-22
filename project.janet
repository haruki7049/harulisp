(declare-project
  :name "harulisp"
  :dependencies ["https://github.com/janet-lang/spork.git"
                 "https://github.com/ianthehenry/judge"])

(declare-executable
  :name "harepl"
  :entry "src/main.janet"
  :install true)
