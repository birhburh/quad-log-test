package TARGET_PACKAGE_NAME;

import android.net.Uri;
import java.io.BufferedWriter;
import java.io.IOException;
import java.io.OutputStream;
import java.io.OutputStreamWriter;
import android.content.ContentResolver;

public class FileOpen {
  public static Uri saved_uri;
  public static boolean first;
  public static MainActivity MainActivity;
  public static ContentResolver resolver;

  public static native void init();

  static native void finish();

  static native void saveUri(byte[] data);

  public FileOpen() {}

  public void finishMainActivity() {
    MainActivity.finish();
  }

  public void OpenFileDialog() {
    MainActivity.OpenFileDialog();
  }

  public void logThis(String text) {
	  FileOpen.writeInFile(text);
  }

  public static void writeInFile(String text) {
    if (saved_uri != null) {
      OutputStream outputStream;
      try {
        String mode = first ? "wt" : "wa";
	first = false;
        outputStream = resolver.openOutputStream(saved_uri, mode);
        BufferedWriter bw = new BufferedWriter(new OutputStreamWriter(outputStream));
        bw.write(text);
        bw.flush();
        bw.close();
      } catch (IOException e) {
        e.printStackTrace();
      }
    }
  }
}
