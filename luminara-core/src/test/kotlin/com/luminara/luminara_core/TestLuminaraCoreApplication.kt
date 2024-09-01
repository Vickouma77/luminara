package com.luminara.luminara_core

import org.springframework.boot.fromApplication
import org.springframework.boot.with


fun main(args: Array<String>) {
	fromApplication<LuminaraCoreApplication>().with(TestcontainersConfiguration::class).run(*args)
}
