use std::fs;

pub fn buildCCpp(header_file_contents: &str, native_file_name: &str) {
    for line in header_file_contents.lines() {
        if (line.contains("JNIEXPORT ") & line.contains(" JNICALL Java_"))
            | line.contains("  (JNIEnv *, jobject")
        {
            if (line.contains("JNIEXPORT ") & line.contains(" JNICALL Java_"))
                | line.contains("  (JNIEnv *, jobject")
            {
                if line.contains("  (JNIEnv *, jobject") {
                    fs::write(
                        // WRITE FOR PARAMETERS
                        native_file_name,
                        format!(
                            "{}{}\n",
                            fs::read_to_string(native_file_name).expect(
                                "\x1b[1;31merror\x1b[0m:  failed to read header file when building"
                            ),
                            line.replace(';', " {\n\n}")
                        ),
                    )
                    .expect("\x1b[1;31merror\x1b[0m: failed to write line when building");
                } else {
                    fs::write(
                        // WRITE FOR DECLARATION
                        native_file_name,
                        format!(
                            "{}\n{}",
                            fs::read_to_string(native_file_name).expect(
                                "\x1b[1;31merror\x1b[0m:  failed to read header file when building"
                            ),
                            line.replace(';', " {\n\n}")
                        ),
                    )
                    .expect("\x1b[1;31merror\x1b[0m: failed to write line when building");
                }
            }
        }
    }
}
