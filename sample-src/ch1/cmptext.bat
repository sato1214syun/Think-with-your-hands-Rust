rem === Windows�̏ꍇ ===
rem  ���ʂ��e�L�X�g�t�@�C���ɏ����o��
python3 fizzbuzz.py > fb_python.txt
rustc fizzbuzz.rs && ./fizzbuzz > fb_rust.txt
rem diff�R�}���h�Ńe�L�X�g���r
fc fb_python.txt fb_rust.txt
