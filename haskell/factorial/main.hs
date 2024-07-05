import System.Environment (getArgs)
import System.Exit (exitSuccess)
import Text.Read (readMaybe)

fac :: (Integral a) => a -> a
fac n
  | n <= -1 = 0
  | n < 2 = 1
  | otherwise = n * fac (n - 1)

main :: IO ()
main = getArgs >>= parse >> exit

parseRun :: String -> IO ()
parseRun x = case readMaybe x :: Maybe Integer of
  Just _x -> print (fac _x)
  Nothing -> return ()

parse :: [String] -> IO ()
parse ["-h"] = usage >> exit
parse [num] = parseRun num
parse _ = usage >> exit

usage :: IO ()
usage = putStrLn "Usage: fac [-h] number"

exit :: IO ()
exit = exitSuccess
