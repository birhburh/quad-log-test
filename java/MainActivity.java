//% IMPORTS

import android.app.Activity;
import android.net.Uri;
import java.io.BufferedWriter;
import java.io.IOException;
import java.io.OutputStream;
import java.io.OutputStreamWriter;

//% END

//% MAIN_ACTIVITY_BODY

private static final int WRITE_REQUEST_CODE = 101;
private Uri saved_uri;

public void OpenFileDialog() { 
    Intent myIntent = new Intent(Intent.ACTION_CREATE_DOCUMENT, null);
    myIntent.setType("text/plain");
    startActivityForResult(myIntent, WRITE_REQUEST_CODE);
}

private void writeInFile(Boolean save, Uri uri, String text) {
        OutputStream outputStream;
        try {
		if (save)
		{
			FileOpen.saveUri(uri.toString().getBytes());
			saved_uri = uri;
		}
	getContentResolver().takePersistableUriPermission(uri, Intent.FLAG_GRANT_READ_URI_PERMISSION);
	String mode = save ? "wt" : "wa";
            outputStream = getContentResolver().openOutputStream(uri, mode);
            BufferedWriter bw = new BufferedWriter(new OutputStreamWriter(outputStream));
            if (save)
		    bw.write(text);
	    else
		    bw.append(text);
            bw.flush();
            bw.close();
        } catch (IOException e) {
            e.printStackTrace();
        }

    }
//% END

//% MAIN_ACTIVITY_ON_ACTIVITY_RESULT

if (requestCode == WRITE_REQUEST_CODE) {

	switch (resultCode) {
                case Activity.RESULT_OK:
                    if (data != null
                            && data.getData() != null) {
                        writeInFile(true, data.getData(), "WRITE DATA\n");
                    }
                    break;
                case Activity.RESULT_CANCELED:
                    break;
            }
	if (saved_uri != null) {
        	writeInFile(false, saved_uri, "FINISH\n");
	}
	FileOpen.finish();
}
//% END



//% MAIN_ACTIVITY_ON_CREATE

FileOpen.MainActivity = this;
FileOpen.init();

//% END

