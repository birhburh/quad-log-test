package TARGET_PACKAGE_NAME;

public class FileOpen {
  private Uri saved_uri;
  public static MainActivity MainActivity;

  public static native void init();

  static native void saveUri(byte[] data);

  static native void finish();

  public FileOpen() {}

  public void OpenFileDialog() {
    MainActivity.OpenFileDialog();
  }
}
