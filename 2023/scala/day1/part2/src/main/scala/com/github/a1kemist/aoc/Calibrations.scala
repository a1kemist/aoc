package com.github.a1kemist.aoc

object Calibrations {
  def solve(lines: Iterator[String]): Int = {
    lines.foldLeft(0) { case (agg, line) => agg + parseLine(line) }
  }

  private[aoc] val Tokens: List[String] = List(
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
  )
  private[aoc] def parseLine(line: String): Int = {
    def recurse(index: Int, maybeFirst: Option[Int], maybeLast: Option[Int]): Int = {
      if(index == line.length) maybeFirst.getOrElse(0) + maybeLast.getOrElse(0)
      else {
        val c = line.charAt(index)
        if(maybeFirst.isEmpty) {
          // check digits first
          if (c.isDigit) recurse(index + 1, Some(c.asDigit * 10), Some(c.asDigit))
          else {
            // check each of the tokens using a lookahead
            Tokens.zipWithIndex.collectFirst {
              case (token, i) if line.regionMatches(index, token, 0, token.length) =>
                (token, i)
            } match {
              case Some((matchingToken, tokenIndex)) =>
                recurse(
                  index + matchingToken.length - 1,
                  Some((tokenIndex + 1) * 10),
                  Some(tokenIndex + 1)
                )
              case None =>
                recurse(
                  index + 1,
                  None,
                  None
                )
            }
          }
        }
        else {
          // check digits first
          if (c.isDigit) recurse(index + 1, maybeFirst, Some(c.asDigit))
          else {
            // check each of the tokens using a lookahead
            Tokens.zipWithIndex.collectFirst {
              case (token, i) if line.regionMatches(index, token, 0, token.length) =>
                (token, i)
            } match {
              case Some((matchingToken, tokenIndex)) =>
                recurse(
                  index + matchingToken.length - 1,
                  maybeFirst,
                  Some(tokenIndex + 1)
                )
              case None =>
                recurse(
                  index + 1,
                  maybeFirst,
                  maybeLast
                )
            }
          }
        }
      }
    }

    recurse(0, Option.empty[Int], Option.empty[Int])
  }
}
