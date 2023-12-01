import sbt.Keys._
import sbt._

object Dependencies {
  object scalaTest {
    val group = "org.scalatest"
    val version = "3.2.17"
    val scalaTest = group %% "scalatest" % version
  }

  lazy val day1Part1Main: Seq[ModuleID] = Seq()
  lazy val day1Part1Test: Seq[ModuleID] = Seq(
    scalaTest.scalaTest
  ).map(_ % Test)
  lazy val day1Part1: Seq[ModuleID] = day1Part1Main ++ day1Part1Test

  lazy val day1Part2: Seq[ModuleID] = day1Part1
}
