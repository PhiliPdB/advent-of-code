module Main where

import Prelude hiding ((<$), (<*), (*>))

import Data.Char (isSpace)
import ParseLib.Abstract


data Expr = Op Char Expr Expr
          | Num Int
    deriving (Show)

whitespace :: Parser Char String
whitespace = greedy (satisfy isSpace)

pExprSimple :: Parser Char Expr
pExprSimple =  Num <$> (whitespace *> integer <* whitespace)
           <|> parenthesised pExpr

pExpr :: Parser Char Expr
-- Part 1
-- pExpr = foldr pExprOp pExprSimple [['*', '+']]
-- Part 2
pExpr = foldr pExprOp pExprSimple [['*'], ['+']]

pExprOp :: [Char] -> Parser Char Expr -> Parser Char Expr
pExprOp ops p = chainl p (Op <$> choice (createOpParsers ops))

createOpParsers :: [Char] -> [Parser Char Char]
createOpParsers = map (\c -> whitespace *> symbol c <* whitespace)

start :: Parser s a -> [s] -> a
start p = fst . head . filter (null . snd) . parse p


evalExpr :: Expr -> Int
evalExpr (Op '*' e1 e2) = evalExpr e1 * evalExpr e2
evalExpr (Op '+' e1 e2) = evalExpr e1 + evalExpr e2
evalExpr (Op _   _  _ ) = undefined
evalExpr (Num n)        = n

main :: IO ()
main = do f <- readFile "./input.txt"
          let parsed = map (evalExpr . start pExpr) (lines f)
          putStrLn $ show $ sum parsed
