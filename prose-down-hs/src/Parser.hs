module Parser(pp, main) where

import Numeric.Natural
import Text.Parsec.Combinator
import Text.Parsec.Prim
import Text.Parsec
import Text.Parsec.Char
import Text.Parsec.String
import Text.ParserCombinators.ReadP (skipSpaces, between)

newtype Document
  = Document [Block]
  deriving (Show)

data Block
  = Paragraph [Inline]
  | Heading [Char] [Inline]
  | LineBreak ()
  | ThemanticBreak ()
  deriving (Show)

data Inline
  = Text Char
  | Tag Slug
  deriving (Show)

newtype Slug = Slug String
  deriving (Show)

document :: Parser Document
document =
  Document <$> many (try heading <|> try paragraph <|> try themanticBreak <|> try lineBreak)

paragraph :: Parser Block
paragraph = Paragraph <$> many1 inline <> (eol *> pure ([]))

themanticBreak :: Parser Block
themanticBreak = ThemanticBreak <$> (newline *> string "---" *> eol *> pure ())

heading :: Parser Block
heading = Heading <$> headingSharp <*> (space *> many1 inline) <> (eol *> pure([]))

headingSharp :: Parser String
headingSharp = count 1 (char '#') <|> count 2 (char '#') <|> count 3 (char '#') <|> count 4 (char '#') <|> count 5 (char '#') <|> count 6 (char '#')

lineBreak :: Parser Block
lineBreak = LineBreak <$> (newline *> pure ())

inline :: Parser Inline
inline = try tag <|> try text

text :: Parser Inline
text = Text <$> anyChar

tag :: Parser Inline
-- tag = Tag <$> (try (space *> tag2) <|> try (tag2 skipSpaces <|> eol))
tag = Tag <$> ((try (spaces *> tag2 <* spaces)) )

tag2 :: Parser Slug
tag2 = char '#' *> slug

slug :: Parser Slug
slug = Slug <$> many1 (letter <|> char '-')

eol :: Parser ()
eol = newline *> pure () <|> eof

runRule :: Show a => Parser a -> String -> IO ()
runRule rule input = case parse rule "" input of
    Left err ->
      putStr "parse error at" >> print err
    Right x -> print x

run :: String -> IO ()
run input =
  case parse document "" input of
    Left err ->
      putStr "parse error at" >> print err
    Right x -> print x

pp :: IO ()
pp = do
  print $ parse document "" "foo\nbar#a"

main :: IO ()
main= putStrLn "someFunc"
