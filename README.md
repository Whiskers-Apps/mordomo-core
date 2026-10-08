# Mordomo Core
## Install
Add jitpack to your `gradle.kts`

```gradle
dependencyResolutionManagement {
  repositoriesMode.set(RepositoriesMode.FAIL_ON_PROJECT_REPOS)
  repositories {
    mavenCentral()
    maven { url = uri("https://jitpack.io") }
  }
}
```

Then add it has a dependency
```gradle
dependencies {
        implementation("com.github.Whiskers-Apps:mordomo-core:1.0.1")
}
```

## Local Install
You can also build it locally.
1. Clone the repository
2. Run `./gradlew publishToMavenLocal`
3. Add the implementation in your project `gradle.kts`
  ```gradle
  implementation("org.whiskersapps:mordomo-core:1.0.1")
  ```
4. Add `mavenLocal()` in `settings.gradle.kts`

  ```gradle
dependencyResolutionManagement {
    repositories {
     ...     
      mavenLocal()
     }
}
  ```
