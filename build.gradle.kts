plugins {
    kotlin("jvm") version "2.3.21"
    `maven-publish`
    alias(libs.plugins.kotlin.serialization)
}

group = "org.whiskersapps"
version = "0.1.0"

repositories {
    mavenCentral()
}

dependencies {
    testImplementation(kotlin("test"))

    implementation(libs.kotlinx.serialization.json)
}

kotlin {
    jvmToolchain(25)
}

publishing{
    publications {
        create<MavenPublication>("maven") {
            from(components["java"])
        }
    }
}

tasks.test {
    useJUnitPlatform()
}