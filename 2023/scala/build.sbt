import sbt.Keys._
import sbt._

val commonSettings = Seq(
    organization := "com.github.a1kemist",
    scalaVersion := "2.13.12"
)

val preventPublicationSettings = Seq(

)

lazy val aoc = (project in file("."))
  .settings(commonSettings: _*)
  .settings(preventPublicationSettings: _*)
  .aggregate(
//    day1,
    day1Part1,
    day1Part2,
//    day2,
//    day3,
//    day4,
//    day5,
//    day6,
//    day7,
//    day8,
//    day9,
//    day10,
//    day11,
//    day12,
//    day13,
//    day14,
//    day15,
//    day16,
//    day17,
//    day18,
//    day19,
//    day20,
//    day21,
//    day22,
//    day23,
//    day24,
//    day25
  )

lazy val day1 = (project in file("day1"))
  .settings(commonSettings: _*)
  .settings(preventPublicationSettings: _*)
  .aggregate(day1Part1, day1Part2)

lazy val day1Part1 = (project in file("day1/part1"))
  .settings(commonSettings: _*)
  .settings(
    libraryDependencies ++= Dependencies.day1Part1
  )

lazy val day1Part2 = (project in file("day1/part2"))
  .settings(commonSettings: _*)
  .settings(
    libraryDependencies ++= Dependencies.day1Part2
  )