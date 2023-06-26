package TARGET_PACKAGE_NAME;

import android.net.Uri;

public class FileOpen {
  public static Uri saved_uri;
  public static MainActivity MainActivity;

  public static native void init();

  static native void saveUri(byte[] data);

  static native void finish();

  public FileOpen() {}

  public void finishMainActivity() {
    MainActivity.finish();
  }

  public void OpenFileDialog() {
    MainActivity.OpenFileDialog();
  }
}
