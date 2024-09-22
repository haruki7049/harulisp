(declare-project
  :name "harulisp"
  :dependencies ["https://github.com/janet-lang/spork.git"
                 "https://github.com/ianthehenry/judge"])

(declare-source
  :source ["src/init.janet"])

(declare-executable
  :name "harepl"
  :entry "bin/harepl.janet"
  :install true)
