# SPDX-License-Identifier: GPL-2.0-only
obj-m += dtld_ib.o

dtld_ib-y := \
	dtld.o \
	dtld_verbs.o \

