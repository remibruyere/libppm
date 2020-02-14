import os
from flask import Flask, flash, request, redirect, url_for, render_template, session, send_from_directory
from werkzeug.utils import secure_filename
from cffi import FFI
ffi = FFI()
ffi.cdef("""
  void invert_image(const char*);  
  void grayscale_image(const char*);  
  void image_gaussian_blur(const char*);  
""")

#Linux
lib = ffi.dlopen("../target/release/libppm.so")
#MaxOS
#lib = ffi.dlopen("../target/release/libppm.dylib")

UPLOAD_FOLDER = 'uploads/'
DOWNLOAD_FOLDER = 'output/'

app = Flask(__name__)
app.secret_key = b'giherjkdhjkfhgkjerhkfjve754'

app.config['UPLOAD_FOLDER'] = UPLOAD_FOLDER
app.config['DOWNLOAD_FOLDER'] = DOWNLOAD_FOLDER

def to_cstring(text):
    return ffi.new("char[]", text.encode('utf-8'))

@app.route('/', methods=['GET', 'POST'])
def index():
        if request.method == 'POST':
                if 'file' not in request.files:
                        return redirect(request.url)
                f = request.files['file']
                if f.filename == "":
                        return redirect(request.url)
                filename = secure_filename(f.filename)
                f.save(os.path.join(app.config['UPLOAD_FOLDER'], filename))
                session['file'] = filename
                if 'output' in session: session.pop('output')
                return redirect(request.url)

        return render_template('index.html')

@app.route('/apply', methods=['POST'])
def apply():
    operation = request.form['operation'] 
    name = to_cstring(session['file'])
    
    if operation == 'grayscale':
        lib.grayscale_image(name)
    elif operation == 'invert':
        lib.invert_image(name)
    if operation == 'gaussian':
        lib.image_gaussian_blur(name)

    session['output'] = session['file']
    return redirect('/')

@app.route('/download', methods=['GET'])
def download():
    return send_from_directory(directory=app.config['DOWNLOAD_FOLDER'], filename=session['output'], as_attachment=True)
    
