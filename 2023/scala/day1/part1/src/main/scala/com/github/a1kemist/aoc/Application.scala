package com.github.a1kemist.aoc

object Application extends App {

  val input = scala.io.Source.fromFile(args(0))
  Calibrations.solve(input.getLines())
}
