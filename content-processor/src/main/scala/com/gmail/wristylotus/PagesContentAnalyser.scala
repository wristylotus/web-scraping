package com.gmail.wristylotus

import com.gmail.wristylotus.tools.Constants
import org.apache.spark.sql.SparkSession
import org.apache.spark.sql.types.{StringType, StructField, StructType}


object PagesContentAnalyser {

  println(Constants.Greeting)

  def main(args: Array[String]): Unit = {
    val spark = SparkSession
      .builder
      .master("local[*]")
      .config("spark.redis.host", "localhost")
      .config("spark.redis.port", "6379")
      .appName("PagesContentAnalyser")
      .getOrCreate()

    val sensors = spark
      .readStream
      .format("redis")
      .option("stream.keys", "crawler:www.rust-lang.org")
      .schema(StructType(Array(
        StructField("_id", StringType),
        StructField("link", StringType),
        StructField("content", StringType)
      )))
      .load()

    val query = sensors
      .writeStream
      .format("console")
      .start()

    query.awaitTermination()
  }
}