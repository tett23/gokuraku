module Main (Main.main) where
import Parser(pp, main )

main :: IO ()
main = do
  Parser.main
  pp
