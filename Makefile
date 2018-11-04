all: src

ATSAML11E16A.svd: patches/svd-compat.diff
	cp ATSAML11E16A.orig.svd ATSAML11E16A.svd
	quilt push patches/svd-compat.diff

src: ATSAML11E16A.svd
	svd2rust -i ATSAML11E16A.svd
	form -i lib.rs -o src && rm lib.rs
	cargo fmt
	quilt push -a

clean:
	rm -rf .pc src
	rm ATSAML11E16A.svd
