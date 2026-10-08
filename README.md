# Mordomo Core
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
