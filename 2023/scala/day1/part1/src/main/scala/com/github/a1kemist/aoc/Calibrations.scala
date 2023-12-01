package com.github.a1kemist.aoc

object Calibrations {
  def solve(lines: Iterator[String]): Int = {
    lines.foldLeft(0) { case (agg, line) => agg + parseLine(line) }
  }

  private[aoc] def parseLine(line: String): Int = {
    val (first, last) = line.foldLeft((Option.empty[Int], Option.empty[Int])){
      case ((maybeFirst, maybeLast), c) => {
        if(maybeFirst.isEmpty) {
          if(c.isDigit) (Some(c.asDigit * 10), Some(c.asDigit))
          else (None, None)
        }
        else {
          if (c.isDigit) (maybeFirst, Some(c.asDigit))
          else (maybeFirst, maybeLast)
        }
      }
    }
    first.getOrElse(0) + last.getOrElse(0)
  }
}
