package com.github.a1kemist.aoc

import org.scalatest.funspec.AnyFunSpec
import org.scalatest.matchers.should.Matchers

class CalibrationsSpec extends AnyFunSpec with Matchers {
  import Calibrations._

  describe("Calibrations"){
    describe("solve"){
      it("should pass the sample input"){
        val input =
          """two1nine
            |eightwothree
            |abcone2threexyz
            |xtwone3four
            |4nineeightseven2
            |zoneight234
            |7pqrstsixteen""".stripMargin

        solve(input.linesIterator) shouldBe 281
      }
      it("should pass the puzzle input"){
        solve(scala.io.Source.fromResource("input-01.txt").getLines()) shouldBe 54770
      }
    }
    describe("parseLine"){
      it("should parse the sample input lines"){
        parseLine("two1nine") shouldBe 29
        parseLine("eightwothree") shouldBe 83
        parseLine("abcone2threexyz") shouldBe 13
        parseLine("xtwone3four") shouldBe 24
        parseLine("4nineeightseven2") shouldBe 42
        parseLine("zoneight234") shouldBe 14
        parseLine("7pqrstsixteen") shouldBe 76
      }
    }
  }
}
