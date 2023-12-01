package com.github.a1kemist.aoc

import org.scalatest.funspec.AnyFunSpec
import org.scalatest.matchers.should.Matchers
class CalibrationsSpec extends AnyFunSpec with Matchers {
  import Calibrations._

  describe("Calibrations"){
    describe("solve"){
      it("should pass the sample input"){
        val input =
          """1abc2
            |pqr3stu8vwx
            |a1b2c3d4e5f
            |treb7uchet""".stripMargin

        solve(input.linesIterator) shouldBe 142
      }
      it("should pass the puzzle input"){
        solve(scala.io.Source.fromResource("input-01.txt").getLines()) shouldBe 54630
      }
    }
    describe("parseLine"){
      it("should parse the sample input lines"){
        parseLine("1abc2") shouldBe 12
        parseLine("pqr3stu8vwx") shouldBe 38
        parseLine("a1b2c3d4e5f") shouldBe 15
        parseLine("treb7uchet") shouldBe 77
      }
    }
  }
}
